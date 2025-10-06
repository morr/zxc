use super::*;

pub struct WorkOnCommandPlugin;

impl Plugin for WorkOnCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<WorkOnCommand>().add_systems(
            Update,
            (
                execute_command,
                monitor_completion,
                handle_internal_interrupts,
                // handle_release_resources,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Message, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct WorkOnCommand {
    pub commandable_entity: Entity,
    pub workable_entity: Entity,
    pub work_kind: WorkKind,
}

fn execute_command(
    mut commands: Commands,
    mut command_reader: MessageReader<WorkOnCommand>,
    mut workable_query: Query<&mut Workable>,
) {
    for command in command_reader.read() {
        match workable_query.get_mut(command.workable_entity) {
            Ok(mut workable) => {
                workable.change_state(
                    WorkableState::BeingWorked(command.clone()),
                    command.workable_entity,
                    &mut commands,
                );
            }
            Err(err) => {
                warn!("Failed to get query result: {:?}", err);
                continue;
            }
        }
    }
}

fn monitor_completion(
    mut commands: Commands,
    mut query: Query<&mut Commandable>,
    mut command_complete_event_reader: MessageReader<WorkCompleteMessage>,
    mut commandable_event_writer: MessageWriter<CommandCompleteMessage>,
) {
    for WorkCompleteMessage {
        commandable_entity,
        workable_entity,
        work_kind,
    } in command_complete_event_reader.read()
    {
        let Ok(mut commandable) = query.get_mut(*commandable_entity) else {
            continue;
        };
        let Some(ref command_type) = commandable.executing else {
            continue;
        };
        let CommandType::WorkOn(command) = command_type else {
            continue;
        };
        if *commandable_entity != command.commandable_entity
            || *workable_entity != command.workable_entity
            || *work_kind != command.work_kind
        {
            continue;
        }

        commandable.complete_executing(
            *commandable_entity,
            &mut commands,
            &mut commandable_event_writer,
        );
    }
}

fn handle_internal_interrupts(
    mut commands: Commands,
    mut event_reader: MessageReader<InternalCommandInterruptMessage>,
    // mut commandable_query: Query<&mut Commandable>,
    mut workable_query: Query<&mut Workable>,
    // mut tasks_scheduler: MessageWriter<ScheduleTaskEvent>,
    // mut work_complete_event_writer: MessageWriter<WorkCompleteEvent>,
) {
    for InternalCommandInterruptMessage(interrupted_command_type) in event_reader.read() {
        if let CommandType::WorkOn(interrupted_command) = interrupted_command_type {
            // let Task(TaskKind::Work {
            //     workable_entity, ..
            // }) = interrupted_command.task
            // else {
            //     panic!("Task kind must be TaskKind::Work");
            // };

            // Handle the workable entity
            if let Ok(mut workable) = workable_query.get_mut(interrupted_command.workable_entity)
                && let WorkableState::BeingWorked(ref worked_command) = workable.state
                && interrupted_command == worked_command
            {
                // tasks_scheduler
                //     .send(ScheduleTaskEvent::push_front(worked_command.task.clone()));
                workable.change_state(
                    WorkableState::Idle,
                    interrupted_command.workable_entity,
                    &mut commands,
                );
            }
        }
    }
}

// fn handle_release_resources(
//     mut event_reader: MessageReader<ReleaseCommandResourcesEvent>,
//     mut tasks_scheduler: MessageWriter<ScheduleTaskEvent>,
// ) {
//     for ReleaseCommandResourcesEvent(interrupted_command_type) in event_reader.read() {
//         if let CommandType::WorkOn(WorkOnCommand { task, .. }) = interrupted_command_type {
//             tasks_scheduler.send(ScheduleTaskEvent::push_front(task.clone()));
//         }
//     }
// }
