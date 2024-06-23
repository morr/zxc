use super::*;

pub struct TaskLockCommandPlugin;

impl Plugin for TaskLockCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TaskLockCommand>().add_systems(
            Update,
            (execute_command, handle_release_resources)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct TaskLockCommand {
    pub commandable_entity: Entity,
    pub task: Task,
}

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<TaskLockCommand>,
    mut commandable_query: Query<&mut Commandable>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
) {
    for TaskLockCommand {
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

        commandable.complete_executing(
            *commandable_entity,
            &mut commands,
            &mut commandable_event_writer,
        );
    }
}

fn handle_release_resources(
    mut event_reader: EventReader<ReleaseCommandResourcesEvent>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
) {
    for ReleaseCommandResourcesEvent(interrupted_command_type) in event_reader.read() {
        if let CommandType::TaskLock(TaskLockCommand { task, .. }) = interrupted_command_type {
            tasks_scheduler.send(ScheduleTaskEvent::push_front(task.clone()));
        }
    }
}
