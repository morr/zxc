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

#[derive(Event, Debug, Clone, Reflect)]
pub struct MoveToCommand(pub Entity, pub IVec2);

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<MoveToCommand>,
    mut query: Query<(
        &Transform,
        &mut Movable,
        &mut Commandable,
        Option<&mut PathfindingTask>,
    )>,
    arc_navmesh: Res<ArcNavmesh>,
    queue_counter: Res<AsyncQueueCounter>,
    // mut movable_state_change_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
) {
    for MoveToCommand(entity, grid_tile) in command_reader.read() {
        // println!("{:?}", MoveToCommand(entity, grid_tile));
        match query.get_mut(*entity) {
            Ok((transform, mut movable, mut commandable, mut maybe_pathfinding_task)) => {
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
                commandable.change_state(CommandableState::Executing, *entity, &mut commands);
            }
            Err(err) => {
                warn!("Failed to get query result: {:?}", err);
                continue;
            }
        }
    }
}

fn monitor_completion(
    mut commands: Commands,
    mut query: Query<&mut Commandable, With<commandable_state::CommandableStateExecutingTag>>,
    mut command_event_reader: EventReader<MovableReachedDestinationEvent>,
    mut commandable_event_writer: EventWriter<CommandExecutedEvent>,
) {
    for MovableReachedDestinationEvent(entity, destination_tile) in command_event_reader.read() {
        // println!("{:?}", MovableReachedDestinationEvent(*entity, *destination_tile));
        let Ok(mut commandable) = query.get_mut(*entity) else {
            continue;
        };
        let Some(ref command_type) = commandable.in_progress else {
            continue;
        };
        let CommandType::MoveTo(MoveToCommand(_entity, move_to_tile)) = command_type else {
            continue;
        };
        if destination_tile != move_to_tile {
            continue;
        }

        commandable.complete_in_progress(*entity, &mut commands, &mut commandable_event_writer);
    }
}
