use super::*;

pub struct ToRestCommandPlugin;

impl Plugin for ToRestCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ToRestCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event, Debug, Clone, Reflect)]
pub struct ToRestCommand(pub Entity);

fn execute_command(
    mut commands: Commands,
    bed_query: Query<&Transform, With<Bed>>,
    mut commandable_query: Query<&mut Commandable>,
    mut command_reader: EventReader<ToRestCommand>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
) {
    for ToRestCommand(entity) in command_reader.read() {
        match commandable_query.get_mut(*entity) {
            Ok(mut commandable) => {
                commandable.complete_executing(
                    *entity,
                    &mut commands,
                    &mut commandable_event_writer,
                );

                if let Some(transform) = bed_query.iter().next() {
                    // either go to bed and sleep there
                    commandable.extend_queue(
                        CommandType::MoveTo(MoveToCommand(
                            *entity,
                            transform.translation.truncate().world_pos_to_grid(),
                        )),
                        *entity,
                        &mut commands,
                    );
                }

                // or sleep at the current spot
                commandable.extend_queue(
                    CommandType::Sleep(SleepCommand(*entity)),
                    *entity,
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
