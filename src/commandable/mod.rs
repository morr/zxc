use crate::*;

expose_submodules!(
    components,
    systems,
    user_selection_command,
    to_rest_command,
    move_to_command
);

pub struct CommandablePlugin;

impl Plugin for CommandablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandExecutedEvent>()
            // .register_type::<Commandable>()
            .add_plugins((
                MoveToCommandPlugin,
                ToRestCommandPlugin,
                UserSelectionCommandPlugin,
            ))
            .add_systems(
                Update,
                process_commands
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                finalize_commands_execution
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
