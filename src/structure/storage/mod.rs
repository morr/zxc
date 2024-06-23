use super::*;

pub struct StoragePlugin;

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Storage>();
    }
}

#[derive(Component, Reflect, Default)]
pub struct Storage;

impl Storage {
    pub fn spawn(
        grid_tile: IVec2,
        commands: &mut Commands,
        texture: Handle<Image>,
        navmesh: &mut Navmesh,
    ) {
        let size = IVec2::new(STORAGE_SIZE, STORAGE_SIZE);

        let id = commands
            .spawn((
                Storage,
                Name::new("Storage"),
                SpriteBundle {
                    texture,
                    sprite: Sprite {
                        custom_size: Some(size.grid_tile_edge_to_world()),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        (grid_tile.grid_tile_edge_to_world()
                            + size.grid_tile_edge_to_world() / 2.0)
                            .extend(STRUCTURE_Z_INDEX),
                    ),
                    ..default()
                },
            ))
            // .insert(ShowAabbGizmo {
            //     colo: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
            // })
            .id();

        navmesh.update_cost(
            (grid_tile.x)..(grid_tile.x + size.x),
            (grid_tile.y)..(grid_tile.y + size.y),
            Navtile::config_cost_to_pathfinding_cost(config().movement_cost.furniture),
        );
        navmesh.add_occupant::<Storage>(&id, grid_tile.x, grid_tile.y);
    }
}

pub fn find_nearest_storage(
    carryable_grid_tile: IVec2,
    storages_query: &Query<(Entity, &Storage, &Transform)>,
) -> Option<(Entity, IVec2)> {
    storages_query
        .iter()
        .map(|(entity, _storage, transform)| {
            let storage_grid_tile = transform.translation.truncate().world_pos_to_grid();
            let distance = carryable_grid_tile.distance_squared(storage_grid_tile);
            (entity, storage_grid_tile, distance)
        })
        .min_by(|(_, _, dist_a), (_, _, dist_b)| {
            dist_a
                .partial_cmp(dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(entity, storage_grid_tile, _)| (entity, storage_grid_tile))
}
