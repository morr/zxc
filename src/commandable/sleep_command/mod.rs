use super::*;

pub struct SleepCommandPlugin;

impl Plugin for SleepCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SleepCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct SleepCommand {
    pub commandable_entity: Entity,
}

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<SleepCommand>,
    mut query: Query<(&mut Pawn, &mut Commandable)>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
) {
    for SleepCommand { commandable_entity } in command_reader.read() {
        // println!("{:?}", SleepCommand { commandable_entity }));
        let Ok((mut pawn, mut commandable)) = query.get_mut(*commandable_entity) else {
            continue;
        };

        pawn.change_state(PawnState::Sleeping, *commandable_entity, &mut commands);

        commandable.complete_executing(
            *commandable_entity,
            &mut commands,
            &mut commandable_event_writer,
        );
        if commandable.state != CommandableState::Idle {
            panic!("Commandable must be in Idle state after SleepCommand")
        }
    }
}
