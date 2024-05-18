use super::*;

#[derive(Event, Debug, Default)]
pub struct UserSelectionCommand(pub Option<UserSelectionData>);

pub fn user_selection_command(
    mut commands: Commands,
    mut current_user_selection: ResMut<CurrentUserSelection>,
    mut user_selection_command_reader: EventReader<UserSelectionCommand>,
    mut user_selection_change_event_writer: EventWriter<UserSelectionChangeEvent>,
) {
    for UserSelectionCommand(maybe_new_selection) in user_selection_command_reader.read() {
        // remove aabb from prev selected
        if let Some(UserSelectionData { id, .. }) = current_user_selection.0 {
            commands.entity(id).remove::<ShowAabbGizmo>();
        }
        // add aabb to to selected
        if let Some(UserSelectionData { id, .. }) = maybe_new_selection {
            commands.entity(*id).insert(ShowAabbGizmo {
                color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
            });
        }

        *current_user_selection = CurrentUserSelection(maybe_new_selection.clone());
        user_selection_change_event_writer.send(UserSelectionChangeEvent);
    }
}
