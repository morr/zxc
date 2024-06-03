use super::*;

pub struct SleepCommandPlugin;

impl Plugin for SleepCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SleepCommand>().add_systems(
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
pub struct SleepCommand {
    pub commandable_entity: Entity,
    pub is_sleep_in_bed: bool,
}

fn execute_command(mut command_reader: EventReader<SleepCommand>, mut query: Query<&mut Restable>) {
    for SleepCommand {
        commandable_entity,
        is_sleep_in_bed,
    } in command_reader.read()
    {
        // println!("{:?}", SleepCommand { commandable_entity }));
        let Ok(mut restable) = query.get_mut(*commandable_entity) else {
            continue;
        };

        restable.change_state(
            RestableState::Resting(Restable::sleep_quality_multiplier(*is_sleep_in_bed)),
            *commandable_entity,
        );
    }
}

fn monitor_completion(
    mut commands: Commands,
    mut query: Query<(&mut Commandable, &mut Restable)>,
    mut command_complete_event_reader: EventReader<RestCompleteEvent>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
) {
    for RestCompleteEvent { commandable_entity } in command_complete_event_reader.read() {
        let Ok((mut commandable, mut restable)) = query.get_mut(*commandable_entity) else {
            continue;
        };
        let Some(ref command_type) = commandable.executing else {
            continue;
        };
        let CommandType::Sleep(SleepCommand {
            commandable_entity: command_commandable_entity,
            ..
        }) = command_type
        else {
            continue;
        };
        if commandable_entity != command_commandable_entity {
            continue;
        }

        commandable.complete_executing(
            *commandable_entity,
            &mut commands,
            &mut commandable_event_writer,
        );
        restable.change_state(RestableState::Activity, *commandable_entity);
    }
}

fn handle_internal_interrupts(
    mut interrupt_reader: EventReader<InternalCommandInterruptEvent>,
    mut query: Query<&mut Restable>,
) {
    for InternalCommandInterruptEvent(interrupted_command) in interrupt_reader.read() {
        if let CommandType::Sleep(SleepCommand {
            commandable_entity, ..
        }) = interrupted_command
        {
            if let Ok(mut restable) = query.get_mut(*commandable_entity) {
                restable.change_state(RestableState::Activity, *commandable_entity);
            }
        }
    }
}
