use crate::*;

expose_submodules!(
    user_selection_command,
    to_rest_command,
    move_to_tile_command
);

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UserSelectionCommand>()
            .add_event::<ToRestCommand>()
            .add_event::<MoveToTileCommand>()
            .add_systems(
                Update,
                (
                    user_selection_command,
                    to_rest_command,
                    move_to_tile_command,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
