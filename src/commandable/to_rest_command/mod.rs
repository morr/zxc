use std::collections::VecDeque;

use super::*;

pub struct ToRestCommandPlugin;

impl Plugin for ToRestCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ToRestCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct ToRestCommand {
    pub commandable_entity: Entity,
}

#[allow(clippy::too_many_arguments)]
fn execute_command(
    mut commands: Commands,
    mut bed_query: Query<(Entity, &Transform, &mut Bed)>,
    mut commandable_query: Query<(&mut Pawn, &mut Commandable)>,
    mut command_reader: EventReader<ToRestCommand>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
    mut available_beds: ResMut<AvailableBeds>,
    mut commandable_interrupt_writer: EventWriter<InternalCommandInterruptEvent>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
) {
    for ToRestCommand { commandable_entity } in command_reader.read() {
        match commandable_query.get_mut(*commandable_entity) {
            Ok((mut pawn, mut commandable)) => {
                commandable.complete_executing(
                    *commandable_entity,
                    &mut commands,
                    &mut commandable_event_writer,
                );
                let sleep_command = CommandType::Sleep(SleepCommand {
                    commandable_entity: *commandable_entity,
                });
                let mut commands_queue = VecDeque::from([sleep_command]);

                if let Some(bed_entity) = pawn.owned_bed {
                    // move to claimed bed
                    let (_entity, transform, _bed) = bed_query.get(bed_entity).unwrap();

                    commands_queue.push_front(CommandType::MoveTo(MoveToCommand {
                        commandable_entity: *commandable_entity,
                        grid_tile: transform.translation.truncate().world_pos_to_grid(),
                    }))
                } else if available_beds.0 > 0 {
                    // or claim fee bed
                    for (bed_entity, transform, mut bed) in bed_query.iter_mut() {
                        if bed.owner.is_none() {
                            bed.claim_by(
                                bed_entity,
                                *commandable_entity,
                                &mut pawn,
                                &mut available_beds,
                            );
                            commands_queue.push_front(CommandType::MoveTo(MoveToCommand {
                                commandable_entity: *commandable_entity,
                                grid_tile: transform.translation.truncate().world_pos_to_grid(),
                            }));

                            break;
                        }
                    }
                }

                commandable.set_queue(
                    commands_queue,
                    *commandable_entity,
                    &mut commands,
                    &mut commandable_interrupt_writer,
                    &mut tasks_scheduler,
                );
            }
            Err(err) => {
                warn!("Failed to get query result: {:?}", err);
                continue;
            }
        }
    }
}

// Since the ToRestCommand is immediately completed in the execute_command function, handling InternalCommandInterruptEvent for SleepCommand is unnecessary. The command is executed and completed within the same system, so there won't be any interruption to handle.
// fn handle_internal_interrupts(
//     mut commands: Commands,
//     mut interrupt_reader: EventReader<InternalCommandInterruptEvent>,
// ) {
// }
