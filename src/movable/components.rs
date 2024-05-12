use super::*;

use std::collections::VecDeque;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub enum MovableState {
    #[default]
    Idle,
    Moving,
    Pathfinding(IVec2),
    PathfindingError,
}

#[derive(Component)]
pub struct MovableMoving;

#[derive(Debug, Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Movable {
    pub path: VecDeque<IVec2>,
    pub speed: f32,
    pub state: MovableState,
}

impl Movable {
    pub fn new(speed: f32) -> Self {
        Self {
            path: VecDeque::new(),
            speed,
            state: MovableState::Idle,
        }
    }

    pub fn to_idle(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        maybe_movable_state_change_event_writer: Option<&mut EventWriter<EntityStateChangeEvent<MovableState>>>,
    ) {
        self.stop_moving(entity, commands);
        self.state = MovableState::Idle;

        if let Some(movable_state_change_event_writer) = maybe_movable_state_change_event_writer {
            movable_state_change_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
        }
    }

    pub fn to_moving(
        &mut self,
        path: VecDeque<IVec2>,
        entity: Entity,
        commands: &mut Commands,
        movable_state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<MovableState>>,
    ) {
        self.state = MovableState::Moving;
        self.path = path;
        commands.entity(entity).insert(MovableMoving);
        movable_state_change_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn to_pathfinding_async(
        &mut self,
        entity: Entity,
        start_tile: IVec2,
        end_tile: IVec2,
        arc_navmesh: &Res<ArcNavmesh>,
        queue_counter: &Res<AsyncQueueCounter>,
        maybe_pathfinding_task: Option<&mut PathfindingTask>,
        commands: &mut Commands,
        movable_state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<MovableState>>,
    ) {
        if self.state == MovableState::Moving {
            self.stop_moving(entity, commands);
        }
        self.state = MovableState::Pathfinding(end_tile);
        movable_state_change_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));

        let navmesh_arc_clone = arc_navmesh.0.clone();
        let task = spawn_async_task(queue_counter, async move {
            let navmesh = navmesh_arc_clone.read().unwrap();

            PathfindingResult {
                start_tile,
                end_tile,
                path: astar_pathfinding(&navmesh, &start_tile, &end_tile),
            }
        });

        if let Some(pathfinding_task) = maybe_pathfinding_task {
            pathfinding_task.push(task);
        } else {
            commands.entity(entity).insert(PathfindingTask::new(task));
        }
    }

    // pub fn to_pathfinding(
    //     &mut self,
    //     entity: Entity,
    //     start_tile: IVec2,
    //     end_tile: IVec2,
    //     commands: &mut Commands,
    //     pathfind_event_writer: &mut EventWriter<PathfindRequestEvent>,
    //     movable_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovableState>>,
    // ) {
    //     if self.state == MovableState::Moving {
    //         self.stop_moving(entity, commands);
    //     }
    //
    //     self.state = MovableState::Pathfinding(end_tile);
    //     pathfind_event_writer.send(PathfindRequestEvent {
    //         start_tile,
    //         end_tile,
    //         entity,
    //     });
    //     movable_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
    // }

    pub fn to_pathfinding_error(
        &mut self,
        entity: Entity,
        movable_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovableState>>,
    ) {
        self.state = MovableState::PathfindingError;
        self.path = [].into();
        movable_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
    }

    fn stop_moving(&mut self, entity: Entity, commands: &mut Commands) {
        self.path = [].into();
        commands.entity(entity).remove::<MovableMoving>();
    }
}
