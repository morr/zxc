use super::*;

pub struct WorkOnCommandPlugin;

impl Plugin for WorkOnCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorkOnCommand>().add_systems(
            Update,
            (execute_command, monitor_completion)
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
    // mut pawn_query: Query<(&mut Pawn, &mut Commandable)>,
    mut workable_query: Query<&mut Workable>,
    // mut event_writer: EventWriter<WorkStartEvent>,
    // mut commandable_event_writer: EventWriter<CommandExecutedEvent>,
    // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for command in command_reader.read() {
        // println!("{:?}", command);
        let workable_entity = command.task.workable_entity;

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
        // println!("{:?}", WorkCompleteEvent { commandable_entity, task });

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
