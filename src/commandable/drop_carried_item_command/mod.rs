use super::*;

pub struct DropCarriedItemCommandPlugin;

impl Plugin for DropCarriedItemCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DropCarriedItemCommand>().add_systems(
            Update,
            (execute_command, handle_release_resources)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct DropCarriedItemCommand {
    pub commandable_entity: Entity,
    pub carryable_entity: Entity,
}

#[allow(clippy::too_many_arguments)]
fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<DropCarriedItemCommand>,
    mut commandable_query: Query<(&mut Pawn, &mut Commandable, &Transform)>,
    mut carryable_query: Query<&mut Carryable>,
    // other_carryables_query: Query<&Carryable>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
    mut commandable_interrupt_writer: EventWriter<ExternalCommandInterruptEvent>,
    assets_collection: Res<AssetsCollection>,
    meshes_collection: Res<MeshesCollection>,
    arc_navmesh: ResMut<ArcNavmesh>,
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


        let grid_tile = transform.world_pos_to_grid();
        let navmesh = arc_navmesh.read();
        let tile_occupants = navmesh
            .get_occupants::<Carryable>(grid_tile.x, grid_tile.y)
            .filter_map(|&entity| carryable_query.get(entity).ok())
            .collect::<Vec<_>>();

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

        carryable.drop_from_inventory(
            &mut pawn,
            *carryable_entity,
            grid_tile,
            &mut commands,
            // &other_carryables_query,
            &assets_collection,
            &meshes_collection,
            &mut arc_navmesh.write(),
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

fn handle_release_resources(
    mut commands: Commands,
    mut event_reader: EventReader<ReleaseCommandResourcesEvent>,
    mut commandable_query: Query<(&mut Pawn, &Transform)>,
    mut carryable_query: Query<&mut Carryable>,
    // other_carryables_query: Query<&Carryable>,
    assets_collection: Res<AssetsCollection>,
    meshes_collection: Res<MeshesCollection>,
    arc_navmesh: ResMut<ArcNavmesh>,
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
            let mut carryable = carryable_query
                .get_mut(*carryable_entity)
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to get query result for carryable_entity {:?} {:?}",
                        carryable_entity, err
                    )
                });

            carryable.drop_from_inventory(
                &mut pawn,
                *carryable_entity,
                transform.world_pos_to_grid(),
                &mut commands,
                // &other_carryables_query,
                &assets_collection,
                &meshes_collection,
                &mut arc_navmesh.write(),
            );
        }
    }
}
