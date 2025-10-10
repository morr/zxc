use super::*;

pub struct CompleteTaskCommandPlugin;

impl Plugin for CompleteTaskCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CompleteTaskCommand>()
            .add_observer(on_release_resources)
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Message, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct CompleteTaskCommand {
    pub commandable_entity: Entity,
    pub task: Task,
}

fn execute_command(
    mut commands: Commands,
    mut command_reader: MessageReader<CompleteTaskCommand>,
    mut commandable_query: Query<&mut Commandable>,
) {
    for CompleteTaskCommand {
        commandable_entity, ..
    } in command_reader.read()
    {
        let mut commandable = match commandable_query.get_mut(*commandable_entity) {
            Ok(commandable) => commandable,
            Err(err) => {
                warn!(
                    "Failed to get query result for commandable_entity {:?}: {:?}",
                    commandable_entity, err
                );
                continue;
            }
        };

        commandable.complete_executing(*commandable_entity, &mut commands);
    }
}

fn on_release_resources(
    event: On<ReleaseCommandResourcesEvent>,
    mut tasks_scheduler: MessageWriter<ScheduleTaskMessage>,
) {
    if let CommandType::CompleteTask(CompleteTaskCommand { task, .. }) = &event.command_type {
        tasks_scheduler.write(ScheduleTaskMessage::push_front(task.clone()));
    }
}
