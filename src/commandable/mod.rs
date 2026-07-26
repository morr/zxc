use crate::*;

mod complete_task_command;
mod components;
mod drop_carried_item_command;
mod feed_command;
mod move_to_command;
mod pick_up_item_command;
mod sleep_command;
mod systems;
mod to_rest_command;
mod user_selection_command;
mod work_on_command;
pub use self::{
    complete_task_command::*, components::*, drop_carried_item_command::*, feed_command::*,
    move_to_command::*, pick_up_item_command::*, sleep_command::*, systems::*, to_rest_command::*,
    user_selection_command::*, work_on_command::*,
};

pub struct CommandablePlugin;

impl Plugin for CommandablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
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
        .add_observer(on_command_complete.run_if(in_state(AppState::Playing)))
        .add_observer(on_interrupt_command.run_if(in_state(AppState::Playing)))
        .add_systems(
            Update,
            process_pending_commands.run_if(in_state(AppState::Playing)),
        );
    }
}

#[macro_export]
macro_rules! interrupt_commandable_commands_queue {
    ($commands:expr, $entity:expr) => {
        $commands.trigger(log_event!(ExternalCommandInterruptEvent {
            entity: $entity
        }));
    };
}
