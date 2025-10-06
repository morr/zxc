use super::*;

pub struct UserSelectionCommandPlugin;

impl Plugin for UserSelectionCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<UserSelectionCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Message, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct UserSelectionCommand(pub Option<UserSelectionData>);

fn execute_command(
    mut commands: Commands,
    mut current_user_selection: ResMut<CurrentUserSelection>,
    mut user_selection_command_reader: MessageReader<UserSelectionCommand>,
    mut user_selection_change_event_writer: MessageWriter<UserSelectionChangeMessage>,
) {
    for command in user_selection_command_reader.read() {
        // println!("{:?", event);
        let maybe_new_selection = &command.0;

        // remove aabb from prev selected
        if let Some(UserSelectionData { entity: id, .. }) = current_user_selection.0 {
            commands.entity(id).remove::<ShowAabbGizmo>();
        }
        // add aabb to to selected
        if let Some(UserSelectionData { entity: id, .. }) = maybe_new_selection {
            commands.entity(*id).insert(ShowAabbGizmo {
                color: Some(Color::srgba(1.0, 1.0, 1.0, 0.25)),
            });
        }

        *current_user_selection = CurrentUserSelection(maybe_new_selection.clone());
        user_selection_change_event_writer.write(log_message!(UserSelectionChangeMessage));
    }
}
