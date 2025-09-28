use crate::*;

expose_submodules!(
    components,
    systems,
    complete_task_command,
    drop_carried_item_command,
    feed_command,
    move_to_command,
    sleep_command,
    pick_up_item_command,
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
            .add_event::<ReleaseCommandResourcesEvent>()
            .register_type::<Commandable>()
            .add_plugins((
                CompleteTaskCommandPlugin,
                DropCarriedItemCommandPlugin,
                FeedCommandPlugin,
                MoveToCommandPlugin,
                SleepCommandPlugin,
                PickUpItemCommandPlugin,
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

#[macro_export]
macro_rules! interrupt_commandable_commands_queue {
    ($writer:expr, $entity:expr) => {
        $writer.write(log_event!(ExternalCommandInterruptEvent($entity)));
    };
}
