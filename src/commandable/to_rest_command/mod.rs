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

fn execute_command(
    mut commands: Commands,
    bed_query: Query<&Transform, With<Bed>>,
    mut commandable_query: Query<&mut Commandable>,
    mut command_reader: EventReader<ToRestCommand>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
) {
    for ToRestCommand { commandable_entity } in command_reader.read() {
        match commandable_query.get_mut(*commandable_entity) {
            Ok(mut commandable) => {
                commandable.complete_executing(
                    *commandable_entity,
                    &mut commands,
                    &mut commandable_event_writer,
                );

                if let Some(transform) = bed_query.iter().next() {
                    // either go to bed and sleep there
                    commandable.extend_queue(
                        CommandType::MoveTo(MoveToCommand{
                            commandable_entity: *commandable_entity,
                            grid_tile: transform.translation.truncate().world_pos_to_grid(),
                        }),
                        *commandable_entity,
                        &mut commands,
                    );
                }

                // or sleep at the current spot
                commandable.extend_queue(
                    CommandType::Sleep(SleepCommand {
                        commandable_entity: *commandable_entity,
                    }),
                    *commandable_entity,
                    &mut commands,
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
