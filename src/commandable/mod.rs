use crate::*;

expose_submodules!(
    components,
    systems,
    move_to_command,
    pick_up_command,
    sleep_command,
    to_rest_command,
    user_selection_command,
    work_on_command
);

pub struct CommandablePlugin;

impl Plugin for CommandablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandCompleteEvent>()
            .add_event::<ExternalCommandInterruptEvent>()
            .add_event::<InternalCommandInterruptEvent>()
            .register_type::<Commandable>()
            .add_plugins((
                MoveToCommandPlugin,
                PickUpCommandPlugin,
                SleepCommandPlugin,
                ToRestCommandPlugin,
                UserSelectionCommandPlugin,
                WorkOnCommandPlugin,
            ))
            .add_systems(
                Update,
                (
                    process_pending_commands,
                    process_complete_commands,
                    process_interrupt_commands,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
