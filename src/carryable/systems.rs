use bevy::sprite::MaterialMesh2dBundle;

use super::*;

pub fn spawn_on_event(
    mut spawn_event_reader: EventReader<SpawnCarryableEvent>,
    mut store_event_writer: EventWriter<StoreCarryableEvent>,
    mut commands: Commands,
    assets_collection: Res<AssetsCollection>,
    meshes_collection: Res<MeshesCollection>,
    mut food: ResMut<FoodStock>,
    arc_navmesh: Res<ArcNavmesh>,
) {
    for SpawnCarryableEvent {
        kind,
        amount,
        grid_tile,
    } in spawn_event_reader.read()
    {
        let component = match *kind {
            CarryableKind::Food => Carryable {
                kind: CarryableKind::Food,
                amount: *amount,
            },
        };

        let carryable_id = commands
            .spawn((
                component,
                MaterialMesh2dBundle {
                    mesh: meshes_collection.food.clone().into(),
                    material: assets_collection.food.clone(),
                    transform: Transform::from_translation(
                        grid_tile.grid_tile_center_to_world().extend(ITEM_Z_INDEX),
                    ),
                    ..default()
                },
            ))
            .id();

        // increment food stock
        match *kind {
            CarryableKind::Food => {
                **food += *amount;
            }
        };

        arc_navmesh
            .write()
            .add_occupant::<Carryable>(carryable_id, grid_tile.x, grid_tile.y);

        store_event_writer.send(log_event!(StoreCarryableEvent {
            entity: carryable_id
        }));
    }
}

pub fn spawn_initial_items(mut event_writer: EventWriter<SpawnCarryableEvent>) {
    event_writer.send(SpawnCarryableEvent {
        kind: CarryableKind::Food,
        amount: config().starting_scene.food,
        grid_tile: IVec2 { x: -8, y: 0 },
    });
}

pub fn store_on_event(
    mut event_reader: EventReader<StoreCarryableEvent>,
    carryable_query: Query<&Transform>,
    storages_query: Query<(Entity, &Storage, &Transform)>,
    mut tasks_queue: ResMut<TasksQueue>,
) {
    for StoreCarryableEvent {
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
                tasks_queue.push_task_back(Task {
                    kind: TaskKind::CarryItem {
                        carryable_entity: *carryable_entity,
                        grid_tile: storage_grid_tile,
                    },
                    grid_tile: carryable_grid_tile,
                });
            }
        } else {
            warn!("Failed to get Carryable: {:?}", carryable_entity);
            continue;
        }
    }
}
