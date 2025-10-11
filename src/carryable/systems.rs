use super::*;

pub fn on_spawn_carryable(
    event: On<SpawnCarryableEvent>,
    mut commands: Commands,
    assets_collection: Res<AssetsCollection>,
    meshes_collection: Res<MeshesCollection>,
    mut food_stock: ResMut<FoodStock>,
    arc_navmesh: Res<ArcNavmesh>,
) {
    let SpawnCarryableEvent {
        kind,
        amount,
        grid_tile,
    } = *event;
    let components = match kind {
        CarryableKind::Food => (
            Carryable {
                kind: CarryableKind::Food,
                amount,
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
            grid_tile,
            &assets_collection,
            &meshes_collection,
        ))
        .id();

    // increment food stock
    if kind == CarryableKind::Food {
        food_stock.amount += amount;
    };

    arc_navmesh
        .write()
        .add_occupant::<Carryable>(&carryable_id, grid_tile.x, grid_tile.y);

    commands.trigger(log_event!(StoreCarryableEvent {
        entity: carryable_id
    }));
}

pub fn spawn_initial_items(mut commands: Commands) {
    let amount = config().starting_scene.food;

    if amount > 0 {
        commands.trigger(log_event!(SpawnCarryableEvent {
            kind: CarryableKind::Food,
            amount: amount / 2,
            grid_tile: IVec2 { x: -12, y: 4 },
        }));

        commands.trigger(log_event!(SpawnCarryableEvent {
            kind: CarryableKind::Food,
            amount: amount / 2,
            grid_tile: IVec2 { x: -15, y: 3 },
        }));
    }
}

pub fn on_store_carryable(
    event: On<StoreCarryableEvent>,
    carryable_query: Query<&Transform>,
    storages_query: Query<(Entity, &Storage, &Transform)>,
    mut tasks_queue: ResMut<TasksQueue>,
) {
    let StoreCarryableEvent {
        entity: carryable_entity,
    } = *event;
    if let Ok(carryable_transform) = carryable_query.get(carryable_entity) {
        let carryable_grid_tile: IVec2 = carryable_transform
            .translation
            .truncate()
            .world_pos_to_grid();

        if let Some((_storage_entity, storage_grid_tile)) =
            find_nearest_storage(carryable_grid_tile, &storages_query)
        {
            tasks_queue.push_task_back(Task(TaskKind::CarryItem {
                carryable_entity,
                destination_grid_tile: storage_grid_tile,
            }));
        }
    } else {
        warn!("Failed to get Carryable: {:?}", carryable_entity);
        return;
    }
}

pub fn on_merge_carryable(
    event: On<MergeCarryablesEvent>,
    mut commands: Commands,
    mut carryables_query: Query<&mut Carryable>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let MergeCarryablesEvent {
        entity_to_merge,
        ref carryable_to_merge,
        grid_tile,
        ref merge_into_entities,
    } = *event;

    for entity in merge_into_entities {
        let Ok(mut carryable) = carryables_query.get_mut(*entity) else {
            continue;
        };
        if carryable.kind != carryable_to_merge.kind {
            continue;
        }
        carryable.amount += carryable_to_merge.amount;
        commands.entity(entity_to_merge).despawn();
        arc_navmesh.write().remove_occupant::<Carryable>(
            &entity_to_merge,
            grid_tile.x,
            grid_tile.y,
        );
        break;
    }
}
