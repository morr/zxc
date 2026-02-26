use super::*;

pub struct CompleteTaskCommandPlugin;

impl Plugin for CompleteTaskCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(execute_command)
            .add_observer(on_release_resources);
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct CompleteTaskCommand {
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

fn on_release_resources(
    event: On<ReleaseCommandResourcesEvent>,
    mut tasks_scheduler: MessageWriter<ScheduleTaskMessage>,
) {
    if let CommandType::CompleteTask(CompleteTaskCommand { task, .. }) = &event.command_type {
        tasks_scheduler.write(ScheduleTaskMessage::push_front(task.clone()));
    }
}
