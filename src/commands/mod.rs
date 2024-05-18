use crate::*;

expose_submodules!(user_selection_command, to_rest_command, move_to_command);

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UserSelectionCommand>()
            .add_event::<ToRestCommand>()
            .add_event::<MoveToCommand>()
            .add_systems(
                Update,
                (
                    execute_user_selection,
                    execute_to_rest,
                    execute_move_to,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
