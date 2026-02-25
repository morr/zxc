use super::*;

pub struct SleepCommandPlugin;

impl Plugin for SleepCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SleepCommand>()
            .add_observer(on_rest_complete)
            .add_observer(on_internal_interrupt)
            .add_systems(
                Update,
                execute_command
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Message, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct SleepCommand {
    pub commandable_entity: Entity,
    pub is_sleep_in_bed: bool,
}

fn execute_command(
    mut command_reader: MessageReader<SleepCommand>,
    mut query: Query<&mut Restable>,
) {
    for SleepCommand {
        commandable_entity,
        is_sleep_in_bed,
    } in command_reader.read()
    {
        let Ok(mut restable) = query.get_mut(*commandable_entity) else {
            continue;
        };

        restable.change_state(
            RestableState::Resting(Restable::sleep_quality_multiplier(*is_sleep_in_bed)),
            *commandable_entity,
        );
    }
}

fn on_rest_complete(
    event: On<RestCompleteEvent>,
    mut commands: Commands,
    mut query: Query<(&mut Commandable, &mut Restable)>,
) {
    let Ok((mut commandable, mut restable)) = query.get_mut(event.entity) else {
        return;
    };
    let Some(ref command_type) = commandable.executing else {
        return;
    };
    let CommandType::Sleep(SleepCommand {
        commandable_entity: command_commandable_entity,
        ..
    }) = command_type
    else {
        return;
    };
    if event.entity != *command_commandable_entity {
        panic!(
            "on_rest_complete: event.entity {:?} != command_commandable_entity {:?}",
            event.entity, command_commandable_entity
        );
    }

    commandable.complete_executing(event.entity, &mut commands);
    restable.change_state(RestableState::Activity, event.entity);
}

fn on_internal_interrupt(
    event: On<InternalCommandInterruptEvent>,
    mut query: Query<&mut Restable>,
) {
    if let CommandType::Sleep(SleepCommand {
        commandable_entity, ..
    }) = event.command_type
        && let Ok(mut restable) = query.get_mut(commandable_entity)
    {
        restable.change_state(RestableState::Activity, commandable_entity);
    }
}
