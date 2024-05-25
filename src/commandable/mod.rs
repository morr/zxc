use crate::*;

expose_submodules!(
    components,
    systems,
    move_to_command,
    sleep_command,
    to_rest_command,
    user_selection_command,
    work_on_command
);

pub struct CommandablePlugin;

impl Plugin for CommandablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandExecutedEvent>()
            .add_event::<CommandAbortedEvent>()
            .register_type::<Commandable>()
            .add_plugins((
                MoveToCommandPlugin,
                SleepCommandPlugin,
                ToRestCommandPlugin,
                UserSelectionCommandPlugin,
                WorkOnCommandPlugin,
            ))
            .add_systems(
                Update,
                process_commands
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                FixedUpdate,
                finalize_commands_execution
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
