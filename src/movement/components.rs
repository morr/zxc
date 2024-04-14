use super::*;

use std::collections::VecDeque;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub enum MovementState {
    #[default]
    Idle,
    Moving,
    Pathfinding(IVec2),
    PathfindingError,
}

#[derive(Component)]
pub struct MovementMoving;

#[derive(Debug, Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Movement {
    pub path: VecDeque<IVec2>,
    pub speed: f32,
    pub state: MovementState,
}

impl Movement {
    pub fn new(speed: f32) -> Self {
        Self {
            path: VecDeque::new(),
            speed,
            state: MovementState::Idle,
        }
    }

    pub fn to_idle(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        movement_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovementState>>,
    ) {
        self.stop_moving(entity, commands);
        self.state = MovementState::Idle;
        movement_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
    }

    pub fn to_moving(
        &mut self,
        path: VecDeque<IVec2>,
        entity: Entity,
        commands: &mut Commands,
        movement_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovementState>>,
    ) {
        self.state = MovementState::Moving;
        self.path = path;
        commands.entity(entity).insert(MovementMoving);
        movement_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
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
        movement_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovementState>>,
    ) {
        if self.state == MovementState::Moving {
            self.stop_moving(entity, commands);
        }
        self.state = MovementState::Pathfinding(end_tile);
        movement_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));

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
    //     movement_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovementState>>,
    // ) {
    //     if self.state == MovementState::Moving {
    //         self.stop_moving(entity, commands);
    //     }
    //
    //     self.state = MovementState::Pathfinding(end_tile);
    //     pathfind_event_writer.send(PathfindRequestEvent {
    //         start_tile,
    //         end_tile,
    //         entity,
    //     });
    //     movement_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
    // }

    pub fn to_pathfinding_error(
        &mut self,
        entity: Entity,
        movement_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovementState>>,
    ) {
        self.state = MovementState::PathfindingError;
        self.path = [].into();
        movement_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
    }

    fn stop_moving(&mut self, entity: Entity, commands: &mut Commands) {
        self.path = [].into();
        commands.entity(entity).remove::<MovementMoving>();
    }
}
