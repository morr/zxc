use super::*;

pub struct WorkOnCommandPlugin;

impl Plugin for WorkOnCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorkOnCommand>().add_systems(
            Update,
            (
                execute_command,
                monitor_completion,
                handle_internal_interrupts,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct WorkOnCommand {
    pub commandable_entity: Entity,
    pub task: Task,
}

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<WorkOnCommand>,
    mut workable_query: Query<&mut Workable>,
) {
    for command in command_reader.read() {
        let TaskKind::Work {
            workable_entity, ..
        } = command.task.kind;

        match workable_query.get_mut(workable_entity) {
            Ok(mut workable) => {
                workable.change_state(
                    WorkableState::BeingWorked(command.clone()),
                    workable_entity,
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
    mut command_complete_event_reader: EventReader<WorkCompleteEvent>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
) {
    for WorkCompleteEvent {
        commandable_entity,
        task,
    } in command_complete_event_reader.read()
    {
        let Ok(mut commandable) = query.get_mut(*commandable_entity) else {
            continue;
        };
        let Some(ref command_type) = commandable.executing else {
            continue;
        };
        let CommandType::WorkOn(WorkOnCommand {
            commandable_entity: command_commandable_entity,
            task: command_task,
        }) = command_type
        else {
            continue;
        };
        if commandable_entity != command_commandable_entity || task != command_task {
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
    mut interrupt_reader: EventReader<InternalCommandInterruptEvent>,
    // mut commandable_query: Query<&mut Commandable>,
    mut workable_query: Query<&mut Workable>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
    // mut work_complete_event_writer: EventWriter<WorkCompleteEvent>,
) {
    for InternalCommandInterruptEvent(interrupted_command_type) in interrupt_reader.read() {
        if let CommandType::WorkOn(interrupted_command) = interrupted_command_type {
            let TaskKind::Work {
                workable_entity, ..
            } = interrupted_command.task.kind;

            // Handle the workable entity
            if let Ok(mut workable) = workable_query.get_mut(workable_entity) {
                if let WorkableState::BeingWorked(ref worked_command) = workable.state {
                    if interrupted_command == worked_command {
                        tasks_scheduler
                            .send(ScheduleTaskEvent::push_front(worked_command.task.clone()));
                        workable.change_state(WorkableState::Idle, workable_entity, &mut commands);
                    }
                }
            }
        }
    }
}
