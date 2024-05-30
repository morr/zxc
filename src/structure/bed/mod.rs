use super::*;

pub struct BedPlugin;

impl Plugin for BedPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bed>().init_resource::<AvailableBeds>();
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct AvailableBeds(pub u32);

#[derive(Component, Reflect, Default)]
pub struct Bed {
    pub owner: Option<Entity>,
}

impl Bed {
    pub fn spawn(
        commands: &mut Commands,
        assets: &Res<TextureAssets>,
        navmesh: &mut Navmesh,
        grid_tile: IVec2,
        // state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<FarmState>>,
    ) {
        let size = IVec2::new(BED_SIZE, BED_SIZE);

        let id = commands
            .spawn((
                Bed::default(),
                Name::new("bed"),
                SpriteBundle {
                    texture: assets.bed.clone(),
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
            Navtile::config_cost_to_pathfinding_cost(CONFIG.movement_cost.furniture),
        );
        navmesh.add_occupation::<Bed>(id, grid_tile.x, grid_tile.y);
    }
}
