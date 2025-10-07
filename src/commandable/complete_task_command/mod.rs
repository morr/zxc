use super::*;

pub struct CompleteTaskCommandPlugin;

impl Plugin for CompleteTaskCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<CompleteTaskCommand>().add_systems(
            Update,
            (execute_command, handle_release_resources)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
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

fn handle_release_resources(
    mut event_reader: MessageReader<ReleaseCommandResourcesMessage>,
    mut tasks_scheduler: MessageWriter<ScheduleTaskMessage>,
) {
    for ReleaseCommandResourcesMessage(interrupted_command_type) in event_reader.read() {
        if let CommandType::CompleteTask(CompleteTaskCommand { task, .. }) =
            interrupted_command_type
        {
            tasks_scheduler.write(ScheduleTaskMessage::push_front(task.clone()));
        }
    }
}
