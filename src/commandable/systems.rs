use super::*;

pub fn process_pending_commands(
    mut commands: Commands,
    mut move_to_tile_command_writer: EventWriter<MoveToCommand>,
    mut sleep_command_writer: EventWriter<SleepCommand>,
    mut to_rest_command_writer: EventWriter<ToRestCommand>,
    mut user_selection_command_writer: EventWriter<UserSelectionCommand>,
    mut work_on_command_writer: EventWriter<WorkOnCommand>,
    mut commandable_query: Query<
        (Entity, &mut Commandable, Option<&mut Pawn>),
        // component tags seems to be working unreliable
        // With<commandable_state::CommandableStatePendingExecutionTag>,
    >,
    // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for (entity, mut commandable, maybe_pawn) in &mut commandable_query {
        continue_unless!(CommandableState::PendingExecution, commandable.state);

        if let Some(command_type) = commandable.start_executing(entity, &mut commands) {
            // println!(
            //     "process_pending_commands {:?} pawn.change_state => PawnState::ExecutingCommand",
            //     &command_type
            // );
            match command_type {
                CommandType::MoveTo(command) => {
                    move_to_tile_command_writer.send(log_event!(command));
                }
                CommandType::Sleep(command) => {
                    sleep_command_writer.send(log_event!(command));
                }
                CommandType::ToRest(command) => {
                    to_rest_command_writer.send(log_event!(command));
                }
                CommandType::UserSelection(command) => {
                    user_selection_command_writer.send(log_event!(command));
                }
                CommandType::WorkOn(command) => {
                    work_on_command_writer.send(log_event!(command));
                }
            }

            if let Some(mut pawn) = maybe_pawn {
                pawn.change_state(
                    PawnState::ExecutingCommand,
                    entity,
                    &mut commands,
                    // &mut pawn_state_change_event_writer,
                );
            }
        }
    }
}

pub fn process_complete_commands(
    mut commands: Commands,
    mut commandable_event_reader: EventReader<CommandCompleteEvent>,
    mut pawn_query: Query<(Option<&mut Pawn>, &Commandable)>,
    // component tags seems to be working unreliable
    // mut pawn_query: Query<
    //     (Option<&mut Pawn>, &Commandable),
    //     (
    //         With<commandable_state::CommandableStateIdleTag>,
    //         With<pawn_state::PawnStateExecutingCommandTag>,
    //     ),
    // >,

    // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for CommandCompleteEvent(entity) in commandable_event_reader.read() {
        // println!("{:?}", CommandCompleteEvent(*entity));
        if let Ok((Some(mut pawn), commandable)) = pawn_query.get_mut(*entity) {
            ensure_state!(PawnState::ExecutingCommand, pawn.state);
            continue_unless!(CommandableState::Idle, commandable.state);

            pawn.change_state(
                PawnState::Idle,
                *entity,
                &mut commands,
                // &mut pawn_state_change_event_writer,
            );
        }
    }
}

pub fn process_interrupt_commands(
    mut commands: Commands,
    mut commandable_event_reader: EventReader<ExternalCommandInterruptEvent>,
    mut pawn_query: Query<(Option<&Pawn>, &mut Commandable)>,
    mut commandable_interrupt_writer: EventWriter<InternalCommandInterruptEvent>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
    // component tags seems to be working unreliable
    // mut pawn_query: Query<
    //     (Option<&Pawn>, &mut Commandable),
    //     (
    //         With<commandable_state::CommandableStateIdleTag>,
    //         With<pawn_state::PawnStateExecutingCommandTag>,
    //     ),
    // >,
    // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for ExternalCommandInterruptEvent(commandable_entity) in commandable_event_reader.read() {
        // println!("{:?}", InterruptCommandEvent(*entity));

        if let Ok((Some(pawn), mut commandable)) = pawn_query.get_mut(*commandable_entity) {
            ensure_state!(PawnState::ExecutingCommand, pawn.state);

            commandable.abort_executing(
                *commandable_entity,
                &mut commands,
                &mut commandable_interrupt_writer,
                &mut tasks_scheduler,
                &mut commandable_event_writer,
            );
        }
    }
}
