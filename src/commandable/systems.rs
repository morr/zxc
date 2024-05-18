use super::*;

pub fn process_commands(
    mut commands: Commands,
    mut user_selection_command_writer: EventWriter<UserSelectionCommand>,
    mut to_rest_command_writer: EventWriter<ToRestCommand>,
    mut move_to_tile_command_writer: EventWriter<MoveToTileCommand>,
    mut commandable_query: Query<
        (Entity, &mut Commandable, Option<&mut Pawn>),
        With<commandable_state::Scheduled>,
    >,
    mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    // for (id, mut commandable, maybe_pawn) in &mut commandable_query {
    //     if let Some(command) = commandable.queue.pop_front() {
    //         match command {
    //             Command::UserSelection(data) => {
    //                 user_selection_command_writer.send(UserSelectionCommand(Some(data)));
    //             }
    //             Command::ToRest(entity) => {
    //                 to_rest_command_writer.send(ToRestCommand(entity));
    //             }
    //             Command::MoveTo(id, grid_tile) => {
    //                 move_to_tile_command_writer.send(MoveToTileCommand { id, grid_tile });
    //             }
    //         }
    //
    //         if let Some(mut pawn) = maybe_pawn {
    //             pawn.change_state(
    //                 PawnState::ExecutingCommandable,
    //                 id,
    //                 &mut commands,
    //                 &mut pawn_state_change_event_writer,
    //             );
    //         }
    //         // Update the state of the entity to indicate it is executing a command
    //         // commands.entity(entity).insert(ExecutingCommand);
    //     }
    // }
}
