use super::*;

pub struct TakeItemCommandPlugin;

impl Plugin for TakeItemCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TakeItemCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct TakeItemCommand {
    pub commandable_entity: Entity,
    pub carryable_entity: Entity,
}

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<TakeItemCommand>,
    mut commandable_query: Query<(&mut Pawn, &mut Commandable, &Transform)>,
    mut carryable_query: Query<(&mut Carryable, &Transform)>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
    mut commandable_interrupt_writer: EventWriter<ExternalCommandInterruptEvent>,
) {
    for TakeItemCommand {
        commandable_entity,
        carryable_entity,
    } in command_reader.read()
    {
        let (mut pawn, mut commandable, commandable_transform) =
            match commandable_query.get_mut(*commandable_entity) {
                Ok((pawn, commandable, transform)) => (pawn, commandable, transform),
                Err(err) => {
                    warn!(
                        "Failed to get query result for commandable_entity {:?}: {:?}",
                        commandable_entity, err
                    );
                    continue;
                }
            };

        let (mut carryable, carryable_transform) = match carryable_query.get_mut(*carryable_entity)
        {
            Ok((carryable, transform)) => (carryable, transform),
            Err(err) => {
                warn!(
                    "Failed to get query result for carryable_entity {:?}: {:?}",
                    carryable_entity, err
                );
                interrupt_commandable_commands_queue!(
                    commandable_interrupt_writer,
                    *commandable_entity
                );
                continue;
            }
        };

        let commandable_grid_tile = commandable_transform.world_pos_to_grid();
        let carryable_grid_tile = carryable_transform.world_pos_to_grid();

        if commandable_grid_tile != carryable_grid_tile {
            warn!("commandable_grid_tile != carryable_grid_tile");
            interrupt_commandable_commands_queue!(
                commandable_interrupt_writer,
                *commandable_entity
            );
            continue;
        }

        carryable.take_into_inventory(&mut pawn, *carryable_entity, &mut commands);

        commandable.complete_executing(
            *commandable_entity,
            &mut commands,
            &mut commandable_event_writer,
        );
    }
}
