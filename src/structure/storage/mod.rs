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
        navmesh.add_occupant::<Bed>(id, grid_tile.x, grid_tile.y);

    }
}
