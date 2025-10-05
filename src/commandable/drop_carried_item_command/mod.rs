use super::*;

pub struct DropCarriedItemCommandPlugin;

impl Plugin for DropCarriedItemCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DropCarriedItemCommand>().add_systems(
            Update,
            (execute_command, handle_release_resources)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Message, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct DropCarriedItemCommand {
    pub commandable_entity: Entity,
    pub carryable_entity: Entity,
}

#[allow(clippy::too_many_arguments)]
fn execute_command(
    mut commands: Commands,
    mut command_reader: MessageReader<DropCarriedItemCommand>,
    mut commandable_query: Query<(&mut Pawn, &mut Commandable, &Transform)>,
    mut carryable_query: Query<&mut Carryable>,
    mut commandable_event_writer: MessageWriter<CommandCompleteEvent>,
    mut commandable_interrupt_writer: MessageWriter<ExternalCommandInterruptEvent>,
    mut merge_carryables_event_writer: MessageWriter<MergeCarryablesEvent>,
    assets_collection: Res<AssetsCollection>,
    meshes_collection: Res<MeshesCollection>,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut food_stock: ResMut<FoodStock>,
) {
    for DropCarriedItemCommand {
        commandable_entity,
        carryable_entity,
    } in command_reader.read()
    {
        let (mut pawn, mut commandable, transform) =
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

        let mut carryable = match carryable_query.get_mut(*carryable_entity) {
            Ok(carryable) => carryable,
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

        pawn.drop_item(
            *carryable_entity,
            &mut carryable,
            transform.world_pos_to_grid(),
            &mut commands,
            &assets_collection,
            &meshes_collection,
            &mut arc_navmesh.write(),
            &mut merge_carryables_event_writer,
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
//     mut event_reader: MessageReader<InternalCommandInterruptEvent>,
// ) {
// }

#[allow(clippy::too_many_arguments)]
fn handle_release_resources(
    mut commands: Commands,
    mut event_reader: MessageReader<ReleaseCommandResourcesEvent>,
    mut commandable_query: Query<(&mut Pawn, &Transform)>,
    mut carryable_query: Query<&mut Carryable>,
    assets_collection: Res<AssetsCollection>,
    meshes_collection: Res<MeshesCollection>,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut merge_carryables_event_writer: MessageWriter<MergeCarryablesEvent>,
    mut food_stock: ResMut<FoodStock>,
) {
    for ReleaseCommandResourcesEvent(interrupted_command_type) in event_reader.read() {
        if let CommandType::DropCarriedItem(DropCarriedItemCommand {
            commandable_entity,
            carryable_entity,
        }) = interrupted_command_type
        {
            let (mut pawn, transform) = commandable_query
                .get_mut(*commandable_entity)
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to get query result for commandable_entity {:?} {:?}",
                        commandable_entity, err
                    )
                });
            let maybe_carryable = carryable_query.get_mut(*carryable_entity);
            let Ok(mut carryable) = maybe_carryable else {
                if pawn.inventory.contains_key(carryable_entity) {
                    panic!(
                        "Failed to get query result for carryable_entity {:?} while pawn {:?} has it in inventory",
                        carryable_entity, pawn
                    )
                } else {
                    continue;
                }
            };

            pawn.drop_item(
                *carryable_entity,
                &mut carryable,
                transform.world_pos_to_grid(),
                &mut commands,
                &assets_collection,
                &meshes_collection,
                &mut arc_navmesh.write(),
                &mut merge_carryables_event_writer,
                &mut food_stock,
            );
        }
    }
}
