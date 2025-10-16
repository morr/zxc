use super::*;

#[allow(clippy::too_many_arguments)]
pub fn process_pending_commands(
    mut commands: Commands,
    mut complete_task_command_writer: MessageWriter<CompleteTaskCommand>,
    mut drop_carried_item_command_writer: MessageWriter<DropCarriedItemCommand>,
    mut feed_command_writer: MessageWriter<FeedCommand>,
    mut move_to_command_writer: MessageWriter<MoveToCommand>,
    mut pick_up_item_command_writer: MessageWriter<PickUpItemCommand>,
    mut sleep_command_writer: MessageWriter<SleepCommand>,
    mut to_rest_command_writer: MessageWriter<ToRestCommand>,
    mut user_selection_command_writer: MessageWriter<UserSelectionCommand>,
    mut work_on_command_writer: MessageWriter<WorkOnCommand>,
    mut commandable_query: Query<
        (Entity, &mut Commandable, Option<&mut Pawn>),
        // component tags seems to be working unreliable
        // With<commandable_state::CommandableStatePendingExecutionTag>,
    >,
    mut pawn_state_change_event_writer: MessageWriter<EntityStateChangeMessage<PawnState>>,
) {
    for (entity, mut commandable, maybe_pawn) in &mut commandable_query {
        continue_unless!(CommandableState::PendingExecution, commandable.state);

        if let Some(command_type) = commandable.start_executing(entity, &mut commands) {
            match command_type {
                CommandType::CompleteTask(command) => {
                    complete_task_command_writer.write(log_message!(command));
                }
                CommandType::DropCarriedItem(command) => {
                    drop_carried_item_command_writer.write(log_message!(command));
                }
                CommandType::Feed(command) => {
                    feed_command_writer.write(log_message!(command));
                }
                CommandType::MoveTo(command) => {
                    move_to_command_writer.write(log_message!(command));
                }
                CommandType::PickUpItem(command) => {
                    pick_up_item_command_writer.write(log_message!(command));
                }
                CommandType::Sleep(command) => {
                    sleep_command_writer.write(log_message!(command));
                }
                CommandType::ToRest(command) => {
                    to_rest_command_writer.write(log_message!(command));
                }
                CommandType::UserSelection(command) => {
                    user_selection_command_writer.write(log_message!(command));
                }
                CommandType::WorkOn(command) => {
                    work_on_command_writer.write(log_message!(command));
                }
            }

            if let Some(mut pawn) = maybe_pawn {
                pawn.change_state(
                    PawnState::ExecutingCommand,
                    entity,
                    &mut commands,
                    &mut pawn_state_change_event_writer,
                );
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
    mut pawn_state_change_event_writer: MessageWriter<EntityStateChangeMessage<PawnState>>,
) {
    // println!("{:?}", CommandCompleteEvent(*entity));

    if let Ok((Some(mut pawn), commandable)) = pawn_query.get_mut(event.entity) {
        ensure_state!(fn: PawnState::ExecutingCommand, pawn.state);
        return_unless!(CommandableState::Idle, commandable.state);

        pawn.change_state(
            PawnState::Idle,
            event.entity,
            &mut commands,
            &mut pawn_state_change_event_writer,
        );
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
    // mut pawn_state_change_event_writer: MessageWriter<EntityStateChangeMessage<PawnState>>,
) {
    if let Ok((Some(pawn), mut commandable)) = pawn_query.get_mut(event.entity) {
        ensure_state!(fn: PawnState::ExecutingCommand, pawn.state);

        commandable.abort_executing(event.entity, &mut commands);
    }
}
