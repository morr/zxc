use super::*;

pub struct PickUpItemCommandPlugin;

impl Plugin for PickUpItemCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PickUpItemCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct PickUpItemCommand {
    pub commandable_entity: Entity,
    pub carryable_entity: Entity,
}

#[allow(clippy::too_many_arguments)]
fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<PickUpItemCommand>,
    mut commandable_query: Query<(&mut Pawn, &mut Commandable, &Transform)>,
    carryable_query: Query<(&Carryable, &Transform)>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
    mut commandable_interrupt_writer: EventWriter<ExternalCommandInterruptEvent>,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut food_stock: ResMut<FoodStock>,
) {
    for PickUpItemCommand {
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

        let (carryable, carryable_transform) = match carryable_query.get(*carryable_entity)
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

        pawn.pick_up_item(
            *carryable_entity,
            carryable.clone(),
            commandable_grid_tile,
            &mut commands,
            &mut arc_navmesh.write(),
            &mut food_stock,
        );

        commandable.complete_executing(
            *commandable_entity,
            &mut commands,
            &mut commandable_event_writer,
        );
    }
}

// The command is executed and completed within the same system (execute_command),
// so there won't be any interruption to handle.
// fn handle_internal_interrupts(
//     mut commands: Commands,
//     mut event_reader: EventReader<InternalCommandInterruptEvent>,
// ) {
// }
