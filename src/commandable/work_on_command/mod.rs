use super::*;

pub struct WorkOnCommandPlugin;

impl Plugin for WorkOnCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorkOnCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event, Debug, Clone, Reflect)]
pub struct WorkOnCommand(pub Entity, pub Task);

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<WorkOnCommand>,
    mut query: Query<(&mut Pawn, &mut Commandable)>,
    mut event_writer: EventWriter<WorkStartEvent>,
    mut commandable_event_writer: EventWriter<CommandExecutedEvent>,
    // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for WorkOnCommand(entity, task) in command_reader.read() {
        // println!("{:?}", WorkOnCommand(*entity, task.clone()));
        match query.get_mut(*entity) {
            Ok((mut pawn, mut commandable)) => {
                commandable.complete_command(
                    *entity,
                    &mut commands,
                    &mut commandable_event_writer,
                );

                pawn.change_state(
                    PawnState::TaskAssigned(task.clone()),
                    *entity,
                    &mut commands,
                    // &mut pawn_state_change_event_writer,
                );
                event_writer.send(WorkStartEvent {
                    pawn_entity: *entity,
                });
            }
            Err(err) => {
                warn!("Failed to get query result: {:?}", err);
                continue;
            }
        }
    }
}
