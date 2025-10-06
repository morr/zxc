use super::*;

pub fn spawn_on_event(
    mut spawn_event_reader: MessageReader<SpawnCarryableMessage>,
    mut store_event_writer: MessageWriter<StoreCarryableMessage>,
    mut commands: Commands,
    assets_collection: Res<AssetsCollection>,
    meshes_collection: Res<MeshesCollection>,
    mut food_stock: ResMut<FoodStock>,
    arc_navmesh: Res<ArcNavmesh>,
) {
    for SpawnCarryableMessage {
        kind,
        amount,
        grid_tile,
    } in spawn_event_reader.read()
    {
        let components = match *kind {
            CarryableKind::Food => (
                Carryable {
                    kind: CarryableKind::Food,
                    amount: *amount,
                },
                CarryableFoodMarker,
            ),
            CarryableKind::InInventory => {
                panic!("Cannot spawn CarryableKind::InInventory")
            }
        };

        let carryable_id = commands
            .spawn(components)
            .insert(Carryable::spawn_mesh_bundle(
                *grid_tile,
                &assets_collection,
                &meshes_collection,
            ))
            .id();

        // increment food stock
        if *kind == CarryableKind::Food {
            food_stock.amount += *amount;
        };

        arc_navmesh
            .write()
            .add_occupant::<Carryable>(&carryable_id, grid_tile.x, grid_tile.y);

        store_event_writer.write(log_event!(StoreCarryableMessage {
            entity: carryable_id
        }));
    }
}

pub fn spawn_initial_items(mut event_writer: MessageWriter<SpawnCarryableMessage>) {
    let amount = config().starting_scene.food;

    if amount > 0 {
        event_writer.write(SpawnCarryableMessage {
            kind: CarryableKind::Food,
            amount: amount / 2,
            grid_tile: IVec2 { x: -12, y: 4 },
        });

        event_writer.write(SpawnCarryableMessage {
            kind: CarryableKind::Food,
            amount: amount / 2,
            grid_tile: IVec2 { x: -15, y: 3 },
        });
    }
}

pub fn store_on_event(
    mut event_reader: MessageReader<StoreCarryableMessage>,
    carryable_query: Query<&Transform>,
    storages_query: Query<(Entity, &Storage, &Transform)>,
    mut tasks_queue: ResMut<TasksQueue>,
) {
    for StoreCarryableMessage {
        entity: carryable_entity,
    } in event_reader.read()
    {
        if let Ok(carryable_transform) = carryable_query.get(*carryable_entity) {
            let carryable_grid_tile: IVec2 = carryable_transform
                .translation
                .truncate()
                .world_pos_to_grid();

            if let Some((_storage_entity, storage_grid_tile)) =
                find_nearest_storage(carryable_grid_tile, &storages_query)
            {
                tasks_queue.push_task_back(Task(TaskKind::CarryItem {
                    carryable_entity: *carryable_entity,
                    destination_grid_tile: storage_grid_tile,
                }));
            }
        } else {
            warn!("Failed to get Carryable: {:?}", carryable_entity);
            continue;
        }
    }
}

pub fn merge_on_event(
    mut commands: Commands,
    mut event_reader: MessageReader<MergeCarryablesMessage>,
    mut carryables_query: Query<&mut Carryable>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    for MergeCarryablesMessage {
        entity_to_merge,
        carryable_to_merge,
        grid_tile,
        merge_into_entities,
    } in event_reader.read()
    {
        for entity in merge_into_entities {
            let Ok(mut carryable) = carryables_query.get_mut(*entity) else {
                continue;
            };
            if carryable.kind != carryable_to_merge.kind {
                continue;
            }
            carryable.amount += carryable_to_merge.amount;
            commands.entity(*entity_to_merge).despawn();
            arc_navmesh.write().remove_occupant::<Carryable>(
                entity_to_merge,
                grid_tile.x,
                grid_tile.y,
            );
            break;
        }
    }
}
