use super::*;

pub struct WorkOnCommandPlugin;

impl Plugin for WorkOnCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(execute_command.run_if(in_state(AppState::Playing)))
            .add_observer(on_internal_interrupt.run_if(in_state(AppState::Playing)))
            .add_observer(on_work_complete.run_if(in_state(AppState::Playing)));
    }
}

#[derive(EntityEvent, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct WorkOnCommand {
    #[event_target]
    pub commandable_entity: Entity,
    pub workable_entity: Entity,
    pub work_kind: WorkKind,
}

fn execute_command(
    event: On<WorkOnCommand>,
    mut commands: Commands,
    mut workable_query: Query<&mut Workable>,
) {
    let command = &*event;

    match workable_query.get_mut(command.workable_entity) {
        Ok(mut workable) => {
            // if workable.work_kind.as_ref() != Some(&command.work_kind) {
            //     interrupt_commandable_commands_queue!(commands, command.commandable_entity);
            //     return;
            // }

            workable.change_state(
                WorkableState::BeingWorked(command.clone()),
                command.workable_entity,
                &mut commands,
            );
        }
        Err(err) => {
            warn!("Failed to get query result: {:?}", err);
        }
    }
}

fn on_work_complete(
    event: On<WorkCompleteEvent>,
    mut commands: Commands,
    mut query: Query<&mut Commandable>,
) {
    let WorkCompleteEvent {
        commandable_entity,
        workable_entity,
        ref work_kind,
    } = *event;

    let Ok(mut commandable) = query.get_mut(commandable_entity) else {
        return;
    };
    let Some(ref command_type) = commandable.executing else {
        return;
    };
    let CommandType::WorkOn(command) = command_type else {
        return;
    };
    if commandable_entity != command.commandable_entity
        || workable_entity != command.workable_entity
        || *work_kind != command.work_kind
    {
        return;
    }

    commandable.complete_executing(commandable_entity, &mut commands);
}

fn on_internal_interrupt(
    event: On<InternalCommandInterruptEvent>,
    mut commands: Commands,
    // mut commandable_query: Query<&mut Commandable>,
    mut workable_query: Query<&mut Workable>,
) {
    if let CommandType::WorkOn(ref interrupted_command) = event.command_type {
        // Handle the workable entity
        if let Ok(mut workable) = workable_query.get_mut(interrupted_command.workable_entity)
            && let WorkableState::BeingWorked(ref worked_command) = workable.state
            && interrupted_command == worked_command
        {
            workable.change_state(
                WorkableState::Idle,
                interrupted_command.workable_entity,
                &mut commands,
            );
        }
    }
}
