use super::*;

pub fn process_pending_commands(
    mut commands: Commands,
    mut commandable_query: Query<
        (Entity, &mut Commandable, Option<&mut Pawn>),
        // component tags seems to be working unreliable
        // With<commandable_state::CommandableStatePendingExecutionTag>,
    >,
) {
    for (entity, mut commandable, maybe_pawn) in &mut commandable_query {
        continue_unless!(CommandableState::PendingExecution, commandable.state);

        if let Some(command_type) = commandable.start_executing(entity, &mut commands) {
            command_type.trigger(&mut commands);

            if let Some(mut pawn) = maybe_pawn {
                pawn.change_state(PawnState::ExecutingCommand, entity, &mut commands);
            }
        }
    }
}

pub fn on_command_complete(
    event: On<CommandCompleteEvent>,
    mut commands: Commands,
    mut pawn_query: Query<(Option<&mut Pawn>, &Commandable)>,
    // component tags seems to be working unreliable
    // mut pawn_query: Query<
    //     (Option<&mut Pawn>, &Commandable),
    //     (
    //         With<commandable_state::CommandableStateIdleTag>,
    //         With<pawn_state::PawnStateExecutingCommandTag>,
    //     ),
    // >,
) {
    // println!("{:?}", CommandCompleteEvent(*entity));

    if let Ok((Some(mut pawn), commandable)) = pawn_query.get_mut(event.entity) {
        ensure_state!(fn: PawnState::ExecutingCommand, pawn.state);
        return_unless!(CommandableState::Idle, commandable.state);

        pawn.change_state(PawnState::Idle, event.entity, &mut commands);
    }
}

pub fn on_interrupt_command(
    event: On<ExternalCommandInterruptEvent>,
    mut commands: Commands,
    mut pawn_query: Query<(Option<&Pawn>, &mut Commandable)>,
    // component tags seems to be working unreliable
    // mut pawn_query: Query<
    //     (Option<&Pawn>, &mut Commandable),
    //     (
    //         With<commandable_state::CommandableStateIdleTag>,
    //         With<pawn_state::PawnStateExecutingCommandTag>,
    //     ),
    // >,
) {
    if let Ok((Some(pawn), mut commandable)) = pawn_query.get_mut(event.entity) {
        ensure_state!(fn: PawnState::ExecutingCommand, pawn.state);

        commandable.abort_executing(event.entity, &mut commands);
    }
}
