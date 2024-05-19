use super::*;

pub struct MoveToCommandPlugin;

impl Plugin for MoveToCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveToCommand>().add_systems(
            Update,
            (execute_command, monitor_completion)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Event, Debug, Clone)]
pub struct MoveToCommand(pub Entity, pub IVec2);

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<MoveToCommand>,
    mut movable_query: Query<(&Transform, &mut Movable, Option<&mut PathfindingTask>)>,
    arc_navmesh: Res<ArcNavmesh>,
    queue_counter: Res<AsyncQueueCounter>,
    // mut movable_state_change_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
) {
    for MoveToCommand(entity, grid_tile) in command_reader.read() {
        // println!("{:?}", command);
        let (transform, mut movable, mut maybe_pathfinding_task) =
            movable_query.get_mut(*entity).unwrap();

        movable.to_pathfinding_async(
            *entity,
            transform.translation.truncate().world_pos_to_grid(),
            *grid_tile,
            &arc_navmesh,
            &queue_counter,
            maybe_pathfinding_task.as_deref_mut(),
            &mut commands,
            // &mut movable_state_change_event_writer,
        );
    }
}

fn monitor_completion(
    mut event_reader: EventReader<MovableReachedDestinationEvent>,
) {
    // for EntityStateChangeEvent(entity, movable_state) in event_reader.read() {
        // println!("{:?}", event);
    // }
}
