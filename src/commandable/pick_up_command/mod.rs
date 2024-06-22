use super::*;

pub struct PickUpCommandPlugin;

impl Plugin for PickUpCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickUpCommand>().add_systems(
            Update,
            (
                execute_command,
                // monitor_completion,
                // handle_internal_interrupts,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct PickUpCommand {
    pub commandable_entity: Entity,
    pub carryable_entity: Entity,
}

fn execute_command(
    mut command_reader: EventReader<PickUpCommand>,
    mut commandable_query: Query<&Transform>,
    mut carryable_query: Query<&Transform>,
    mut commandable_interrupt_writer: EventWriter<ExternalCommandInterruptEvent>,
) {
    for PickUpCommand {
        commandable_entity,
        carryable_entity,
    } in command_reader.read()
    {
        let commandable_transform = match commandable_query.get_mut(*commandable_entity) {
            Ok(transform) => transform,
            Err(err) => {
                warn!(
                    "Failed to get query result for commandable_entity {:?}: {:?}",
                    commandable_entity, err
                );
                continue;
            }
        };

        let carryable_transform = match carryable_query.get_mut(*carryable_entity) {
            Ok(transform) => transform,
            Err(err) => {
                warn!(
                    "Failed to get query result for carryable_entity {:?}: {:?}",
                    carryable_entity, err
                );
                continue;
            }
        };

        let commandable_grid_tile = commandable_transform.world_pos_to_grid();
        let carryable_grid_tile = carryable_transform.world_pos_to_grid();

        if commandable_grid_tile != carryable_grid_tile {
            warn!("commandable_grid_tile != carryable_grid_tile");
            commandable_interrupt_writer.send(log_event!(ExternalCommandInterruptEvent(
                *commandable_entity
            )));
            continue;
        }
    }
}

// fn monitor_completion(
//     mut commands: Commands,
//     mut query: Query<&mut Commandable>,
//     mut command_complete_event_reader: EventReader<MovableReachedDestinationEvent>,
//     mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
// ) {
//     for MovableReachedDestinationEvent(entity, destination_tile) in
//         command_complete_event_reader.read()
//     {
//         // println!("{:?}", MovableReachedDestinationEvent(*entity, *destination_tile));
//         let Ok(mut commandable) = query.get_mut(*entity) else {
//             continue;
//         };
//         let Some(ref command_type) = commandable.executing else {
//             continue;
//         };
//         let CommandType::PickUp(PickUpCommand {
//             commandable_entity: _,
//             grid_tile: move_to_tile,
//         }) = command_type
//         else {
//             continue;
//         };
//         if destination_tile != move_to_tile {
//             continue;
//         }
//
//         commandable.complete_executing(*entity, &mut commands, &mut commandable_event_writer);
//     }
// }
//
// fn handle_internal_interrupts(
//     mut commands: Commands,
//     mut interrupt_reader: EventReader<InternalCommandInterruptEvent>,
//     mut query: Query<&mut Movable>,
// ) {
//     for InternalCommandInterruptEvent(interrupted_command) in interrupt_reader.read() {
//         let CommandType::PickUp(PickUpCommand {
//             commandable_entity,
//             grid_tile: commanding_to_tile,
//         }) = interrupted_command
//         else {
//             continue;
//         };
//         let Ok(mut movable) = query.get_mut(*commandable_entity) else { continue };
//
//         if let MovableState::Moving(moving_to_tile) | MovableState::Pathfinding(moving_to_tile) | MovableState::PathfindingError(moving_to_tile) = movable.state {
//             if moving_to_tile == *commanding_to_tile {
//                 movable.to_idle(*commandable_entity, &mut commands, None);
//             } else {
//                 warn!(
//                     "Attempt to interrupt PickUp({:?}) for Movable({:?})",
//                     commanding_to_tile, moving_to_tile
//                 );
//             }
//         }
//     }
// }
