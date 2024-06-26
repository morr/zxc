use super::*;

use std::collections::VecDeque;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub enum MovableState {
    #[default]
    Idle,
    Moving(IVec2),
    Pathfinding(IVec2),
    PathfindingError(IVec2),
}

#[derive(Component, Reflect)]
pub struct MovableStateMovinTag;

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Movable {
    pub speed: f32,
    pub path: VecDeque<IVec2>,
    pub state: MovableState,
}

#[derive(Event, Debug, Clone)]
pub struct MovableReachedDestinationEvent(pub Entity, pub IVec2);

impl Movable {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            path: VecDeque::new(),
            state: MovableState::Idle,
        }
    }

    pub fn to_idle(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        maybe_event_writer: Option<&mut EventWriter<MovableReachedDestinationEvent>>,
        // maybe_movable_state_change_event_writer: Option<&mut EventWriter<EntityStateChangeEvent<MovableState>>>,
    ) {
        if self.path.is_empty() {
            if let MovableState::Moving(end_tile) | MovableState::Pathfinding(end_tile) = self.state
            {
                if let Some(event_writer) = maybe_event_writer {
                    event_writer.send(log_event!(MovableReachedDestinationEvent(entity, end_tile)));
                }
            }
        }

        self.stop_moving(entity, commands);
        self.state = MovableState::Idle;

        // if let Some(movable_state_change_event_writer) = maybe_movable_state_change_event_writer {
        //     movable_state_change_event_writer.send(log_event!(EntityStateChangeEvent(entity, self.state.clone())));
        // }
    }

    pub fn to_moving(
        &mut self,
        end_tile: IVec2,
        path: VecDeque<IVec2>,
        entity: Entity,
        commands: &mut Commands,
        // movable_state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<MovableState>>,
    ) {
        self.state = MovableState::Moving(end_tile);
        self.path = path;
        commands.entity(entity).insert(MovableStateMovinTag);
        // movable_state_change_event_writer.send(log_event!(EntityStateChangeEvent(entity, self.state.clone())));
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
        // movable_state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<MovableState>>,
    ) {
        // println!("MovableState {:?}=>{:?}", self.state, MovableState::Pathfinding(end_tile));
        self.stop_moving(entity, commands);
        self.state = MovableState::Pathfinding(end_tile);
        // movable_state_change_event_writer.send(log_event!(EntityStateChangeEvent(entity, self.state.clone())));

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
    //     pathfind_event_writer.send(log_event!(PathfindRequestEvent {
    //         start_tile,
    //         end_tile,
    //         entity,
    //     }));
    //     movable_state_event_writer.send(log_event!(EntityStateChangeEvent(entity, self.state.clone())));
    // }

    pub fn to_pathfinding_error(
        &mut self,
        entity: Entity,
        end_tile: IVec2,
        commands: &mut Commands,
        // movable_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovableState>>,
    ) {
        // println!("MovableState {:?}=>{:?}", self.state, MovableState::PathfindingError(end_tile));
        self.stop_moving(entity, commands);
        self.state = MovableState::PathfindingError(end_tile);
        // movable_state_event_writer.send(log_event!(EntityStateChangeEvent(entity, self.state.clone())));
    }

    fn stop_moving(&mut self, entity: Entity, commands: &mut Commands) {
        self.path = [].into();
        commands.entity(entity).remove::<MovableStateMovinTag>();
    }
}
