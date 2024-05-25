use super::*;

pub struct SleepCommandPlugin;

impl Plugin for SleepCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SleepCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event, Debug, Clone, Reflect)]
pub struct SleepCommand(pub Entity);

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<SleepCommand>,
    mut query: Query<(&mut Pawn, &mut Commandable)>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
) {
    for SleepCommand(entity) in command_reader.read() {
        // println!("{:?}", SleepCommand(*entity));
        let Ok((mut pawn, mut commandable)) = query.get_mut(*entity) else {
            continue;
        };

        pawn.change_state(PawnState::Sleeping, *entity, &mut commands);

        commandable.complete_executing(*entity, &mut commands, &mut commandable_event_writer);
        if commandable.state != CommandableState::Idle {
            panic!("Commandable must be in Idle state after SleepCommand")
        }
    }
}
