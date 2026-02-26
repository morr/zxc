use super::*;

#[allow(clippy::too_many_arguments)]
pub fn process_pending_commands(
    mut commands: Commands,
    mut drop_carried_item_command_writer: MessageWriter<DropCarriedItemCommand>,
    mut pick_up_item_command_writer: MessageWriter<PickUpItemCommand>,
    mut to_rest_command_writer: MessageWriter<ToRestCommand>,
    mut work_on_command_writer: MessageWriter<WorkOnCommand>,
    mut commandable_query: Query<
        (Entity, &mut Commandable, Option<&mut Pawn>),
        // component tags seems to be working unreliable
        // With<commandable_state::CommandableStatePendingExecutionTag>,
    >,
) {
    for (entity, mut commandable, maybe_pawn) in &mut commandable_query {
        continue_unless!(CommandableState::PendingExecution, commandable.state);

        if let Some(command_type) = commandable.start_executing(entity, &mut commands) {
            match command_type {
                CommandType::CompleteTask(command) => {
                    commands.trigger(log_event!(command));
                }
                CommandType::DropCarriedItem(command) => {
                    drop_carried_item_command_writer.write(log_message!(command));
                }
                CommandType::Feed(command) => {
                    commands.trigger(log_event!(command));
                }
                CommandType::MoveTo(command) => {
                    commands.trigger(log_event!(command));
                }
                CommandType::PickUpItem(command) => {
                    pick_up_item_command_writer.write(log_message!(command));
                }
                CommandType::Sleep(command) => {
                    commands.trigger(log_event!(command));
                }
                CommandType::ToRest(command) => {
                    to_rest_command_writer.write(log_message!(command));
                }
                CommandType::UserSelection(command) => {
                    commands.trigger(log_event!(command));
                }
                CommandType::WorkOn(command) => {
                    work_on_command_writer.write(log_message!(command));
                }
            }

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
