use super::*;

pub struct DropCarriedItemCommandPlugin;

impl Plugin for DropCarriedItemCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(execute_command.run_if(in_state(AppState::Playing)))
            .add_observer(on_release_resources.run_if(in_state(AppState::Playing)));
    }
}

#[derive(EntityEvent, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct DropCarriedItemCommand {
    #[event_target]
    pub commandable_entity: Entity,
    pub carryable_entity: Entity,
}

#[allow(clippy::too_many_arguments)]
fn execute_command(
    event: On<DropCarriedItemCommand>,
    mut commands: Commands,
    mut commandable_query: Query<(&mut Commandable, &Transform), With<Pawn>>,
    carryable_query: Query<(&Carryable, Option<&CarriedBy>)>,
    assets_collection: Res<AssetsCollection>,
    meshes_collection: Res<MeshesCollection>,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut food_stock: ResMut<FoodStock>,
) {
    let DropCarriedItemCommand {
        commandable_entity,
        carryable_entity,
    } = *event;

    let (mut commandable, transform) = match commandable_query.get_mut(commandable_entity) {
        Ok((commandable, transform)) => (commandable, transform),
        Err(err) => {
            warn!(
                "Failed to get query result for commandable_entity {:?}: {:?}",
                commandable_entity, err
            );
            return;
        }
    };

    let (carryable, carried_by) = match carryable_query.get(carryable_entity) {
        Ok(result) => result,
        Err(err) => {
            warn!(
                "Failed to get query result for carryable_entity {:?}: {:?}",
                carryable_entity, err
            );
            interrupt_commandable_commands_queue!(commands, commandable_entity);
            return;
        }
    };

    carryable.drop_off(
        carryable_entity,
        carried_by,
        commandable_entity,
        transform.world_pos_to_grid(),
        &mut commands,
        &assets_collection,
        &meshes_collection,
        &mut arc_navmesh.write(),
        &mut food_stock,
    );

    commandable.complete_executing(commandable_entity, &mut commands);
}

// The command is executed and completed within the same system (execute_command),
// so there won't be any interruption to handle.
// fn on_internal_interrupts(
//     event: On<InternalCommandInterruptEvent>,
//     mut commands: Commands,
// ) {
// }

#[allow(clippy::too_many_arguments)]
fn on_release_resources(
    event: On<ReleaseCommandResourcesEvent>,
    mut commands: Commands,
    commandable_query: Query<(&Transform, Option<&Carrying>), With<Pawn>>,
    carryable_query: Query<(&Carryable, Option<&CarriedBy>)>,
    assets_collection: Res<AssetsCollection>,
    meshes_collection: Res<MeshesCollection>,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut food_stock: ResMut<FoodStock>,
) {
    if let CommandType::DropCarriedItem(DropCarriedItemCommand {
        commandable_entity,
        carryable_entity,
    }) = event.command_type
    {
        let (transform, carrying) =
            commandable_query
                .get(commandable_entity)
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to get query result for commandable_entity {:?} {:?}",
                        commandable_entity, err
                    )
                });
        let Ok((carryable, carried_by)) = carryable_query.get(carryable_entity) else {
            if carrying.is_some_and(|carrying| carrying.contains(carryable_entity)) {
                panic!(
                    "Failed to get query result for carryable_entity {:?} while pawn {:?} is carrying it",
                    carryable_entity, commandable_entity
                )
            } else {
                return;
            }
        };

        carryable.drop_off(
            carryable_entity,
            carried_by,
            commandable_entity,
            transform.world_pos_to_grid(),
            &mut commands,
            &assets_collection,
            &meshes_collection,
            &mut arc_navmesh.write(),
            &mut food_stock,
        );
    }
}
