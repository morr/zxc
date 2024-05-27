use super::*;

pub struct MoveToCommandPlugin;

impl Plugin for MoveToCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveToCommand>().add_systems(
            Update,
            (
                execute_command,
                monitor_completion,
                handle_internal_interrupts,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct MoveToCommand {
    pub commandable_entity: Entity,
    pub grid_tile: IVec2,
}

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<MoveToCommand>,
    mut query: Query<(&Transform, &mut Movable, Option<&mut PathfindingTask>)>,
    arc_navmesh: Res<ArcNavmesh>,
    queue_counter: Res<AsyncQueueCounter>,
    // mut movable_state_change_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
) {
    for MoveToCommand {
        commandable_entity,
        grid_tile,
    } in command_reader.read()
    {
        // println!("{:?}", MoveToCommand(entity, grid_tile));
        match query.get_mut(*commandable_entity) {
            Ok((transform, mut movable, mut maybe_pathfinding_task)) => {
                movable.to_pathfinding_async(
                    *commandable_entity,
                    transform.translation.truncate().world_pos_to_grid(),
                    *grid_tile,
                    &arc_navmesh,
                    &queue_counter,
                    maybe_pathfinding_task.as_deref_mut(),
                    &mut commands,
                    // &mut movable_state_change_event_writer,
                );
                // commandable.change_state(CommandableState::Executing, *entity, &mut commands);
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
    mut query: Query<&mut Commandable>,
    mut command_complete_event_reader: EventReader<MovableReachedDestinationEvent>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
) {
    for MovableReachedDestinationEvent(entity, destination_tile) in
        command_complete_event_reader.read()
    {
        // println!("{:?}", MovableReachedDestinationEvent(*entity, *destination_tile));
        let Ok(mut commandable) = query.get_mut(*entity) else {
            continue;
        };
        let Some(ref command_type) = commandable.executing else {
            continue;
        };
        let CommandType::MoveTo(MoveToCommand {
            commandable_entity: _,
            grid_tile: move_to_tile,
        }) = command_type
        else {
            continue;
        };
        if destination_tile != move_to_tile {
            continue;
        }

        commandable.complete_executing(*entity, &mut commands, &mut commandable_event_writer);
    }
}

fn handle_internal_interrupts(
    mut commands: Commands,
    mut interrupt_reader: EventReader<InternalCommandInterruptEvent>,
    mut query: Query<&mut Movable>,
) {
    for InternalCommandInterruptEvent(interrupted_command) in interrupt_reader.read() {
        let CommandType::MoveTo(MoveToCommand {
            commandable_entity,
            grid_tile: commanding_to_tile,
        }) = interrupted_command
        else {
            continue;
        };
        let Ok(mut movable) = query.get_mut(*commandable_entity) else { continue };

        if let MovableState::Moving(moving_to_tile) | MovableState::Pathfinding(moving_to_tile) | MovableState::PathfindingError(moving_to_tile) = movable.state {
            if moving_to_tile == *commanding_to_tile {
                movable.to_idle(*commandable_entity, &mut commands, None);
            } else {
                warn!(
                    "Attempt to interrupt MoveTo({:?}) for Movable({:?})",
                    commanding_to_tile, moving_to_tile
                );
            }
        }
    }
}
