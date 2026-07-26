use crate::commandable::CommandType;
use crate::commandable::Commandable;
use crate::commandable::ReleaseCommandResourcesEvent;
use crate::prelude::*;
use crate::tasks_queue::ScheduleTaskEvent;
use crate::tasks_queue::Task;
pub struct CompleteTaskCommandPlugin;

impl Plugin for CompleteTaskCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(execute_command.run_if(in_state(AppState::Playing)))
            .add_observer(on_release_resources.run_if(in_state(AppState::Playing)));
    }
}

#[derive(EntityEvent, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct CompleteTaskCommand {
    #[event_target]
    pub commandable_entity: Entity,
    pub task: Task,
}

fn execute_command(
    event: On<CompleteTaskCommand>,
    mut commands: Commands,
    mut commandable_query: Query<&mut Commandable>,
) {
    let CompleteTaskCommand {
        commandable_entity, ..
    } = *event;

    let mut commandable = match commandable_query.get_mut(commandable_entity) {
        Ok(commandable) => commandable,
        Err(err) => {
            warn!(
                "Failed to get query result for commandable_entity {:?}: {:?}",
                commandable_entity, err
            );
            return;
        }
    };

    commandable.complete_executing(commandable_entity, &mut commands);
}

fn on_release_resources(event: On<ReleaseCommandResourcesEvent>, mut commands: Commands) {
    if let CommandType::CompleteTask(CompleteTaskCommand { task, .. }) = &event.command_type {
        commands.trigger(log_event!(ScheduleTaskEvent::push_front(task.clone())));
    }
}
