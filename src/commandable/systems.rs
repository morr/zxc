use super::*;

pub fn process_commands(
    mut commands: Commands,
    mut move_to_tile_command_writer: EventWriter<MoveToCommand>,
    mut to_rest_command_writer: EventWriter<ToRestCommand>,
    mut user_selection_command_writer: EventWriter<UserSelectionCommand>,
    mut work_on_command_writer: EventWriter<WorkOnCommand>,
    mut commandable_query: Query<
        (Entity, &mut Commandable, Option<&mut Pawn>),
        With<commandable_state::CommandableStatePendingExecutionMarker>,
    >,
    // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for (entity, mut commandable, maybe_pawn) in &mut commandable_query {
        if commandable.state != CommandableState::PendingExecution {
            debug!("process_commands>> got CommandableState::{:?} while expected CommandableState::{:?} by Query<With<commandable_state::CommandableStatePendingExecutionMarker>> param", commandable.state, CommandableState::PendingExecution);
            continue;
        }

        if let Some(command_type) = commandable.queue.pop_front() {
            commandable.in_progress = Some(command_type.clone());

            match command_type {
                CommandType::MoveTo(command) => {
                    move_to_tile_command_writer.send(command);
                }
                CommandType::ToRest(command) => {
                    to_rest_command_writer.send(command);
                }
                CommandType::UserSelection(command) => {
                    user_selection_command_writer.send(command);
                }
                CommandType::WorkOn(command) => {
                    work_on_command_writer.send(command);
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

pub fn finalize_commands_execution(
    mut commands: Commands,
    mut commandable_event_reader: EventReader<CommandExecutedEvent>,
    mut pawn_query: Query<
        (Option<&mut Pawn>, &Commandable),
        (
            With<commandable_state::CommandableStateIdleMarker>,
            With<pawn_state::PawnStateExecutingCommandMarker>,
        ),
    >,
    // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for CommandExecutedEvent(entity) in commandable_event_reader.read() {
        if let Ok((Some(mut pawn), commandable)) = pawn_query.get_mut(*entity) {
            if pawn.state != PawnState::ExecutingCommand {
                debug!("finalize_commands_execution>> got PawnState::{:?} while expected PawnState::{:?} by Query<With<pawn_state::PawnStateIdleMarker>> param", pawn.state, PawnState::ExecutingCommand);
                continue;
            }
            if commandable.state != CommandableState::Idle {
                debug!("finalize_commands_execution>> got CommandableState::{:?} while expected CommandableState::{:?} by Query<With<pawn_state::PawnStateExecutingCommandMarker>> param", commandable.state, CommandableState::Idle);
                continue;
            }

            pawn.change_state(
                PawnState::Idle,
                *entity,
                &mut commands,
                // &mut pawn_state_change_event_writer,
            );
        }
    }
}
