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

#[derive(Event, Debug, Clone, Reflect)]
pub struct WorkOnCommand(pub Entity, pub Task);

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<WorkOnCommand>,
    // mut pawn_query: Query<(&mut Pawn, &mut Commandable)>,
    mut workable_query: Query<&mut Workable>,
    // mut event_writer: EventWriter<WorkStartEvent>,
    // mut commandable_event_writer: EventWriter<CommandExecutedEvent>,
    // mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for WorkOnCommand(commandable_entity, task) in command_reader.read() {
        println!("{:?}", WorkOnCommand(*commandable_entity, task.clone()));

        match workable_query.get_mut(task.workable_entity) {
            Ok(mut workable) => {
                workable.change_state(
                    WorkableState::BeingWorked(*commandable_entity, task.clone()),
                    task.workable_entity,
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
    mut commandable_event_writer: EventWriter<CommandExecutedEvent>,
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
        let CommandType::WorkOn(WorkOnCommand(command_commandable_entity, command_task)) =
            command_type
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
