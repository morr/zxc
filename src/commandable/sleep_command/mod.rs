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
    mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for SleepCommand { commandable_entity } in command_reader.read() {
        // println!("{:?}", SleepCommand { commandable_entity }));
        let Ok((mut pawn, mut commandable)) = query.get_mut(*commandable_entity) else {
            continue;
        };

        pawn.change_state(
            PawnState::Sleeping,
            *commandable_entity,
            &mut commands,
            &mut pawn_state_change_event_writer,
        );

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

// Since the SleepCommand is immediately completed in the execute_command function, handling InternalCommandInterruptEvent for SleepCommand is unnecessary. The command is executed and completed within the same system, so there won't be any interruption to handle.
// fn handle_internal_interrupts(
//     mut commands: Commands,
//     mut interrupt_reader: EventReader<InternalCommandInterruptEvent>,
//     mut query: Query<&mut Pawn>,
// ) {
//     for InternalCommandInterruptEvent(interrupted_command) in interrupt_reader.read() {
//         if let CommandType::Sleep(SleepCommand { commandable_entity }) = interrupted_command {
//             if let Ok(mut pawn) = query.get_mut(*commandable_entity) {
//                 pawn.change_state(PawnState::Idle, *commandable_entity, &mut commands);
//             }
//         }
//     }
// }
