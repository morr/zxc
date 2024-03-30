use super::*;

pub fn spawn_base(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut navmesh: ResMut<Navmesh>,
) {
    // https://fin-nio.itch.io/pixel-houses
    let texture_handle = asset_server.load("sprites/castle_complete.png");
    let grid_pos = IVec2::new(0, 0);

    commands
        .spawn(StructureBundle {
            structure: Structure {},
            name: Name::new("Base"),
            sprite_bundle: SpriteBundle {
                texture: texture_handle.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BASE_WIDTH * TILE_SIZE, BASE_HEIGHT * TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(
                    grid_pos.grid_tile_edge_to_world().extend(STRUCTURE_Z_INDEX),
                ),
                ..default()
            },
        })
        .insert(ShowAabbGizmo {
            color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
        });

    // mark navmesh tiles as occupied
    for x in (grid_pos.x - (BASE_WIDTH / 2.0) as i32)..(grid_pos.x + (BASE_WIDTH / 2.0) as i32) {
        for y in
            (grid_pos.y - (BASE_HEIGHT / 2.0) as i32)..(grid_pos.x + (BASE_HEIGHT / 2.0) as i32)
        {
            if let Some(navtile) = navmesh.navtiles.get_mut(x, y) {
                navtile.passable = false
            } else {
                error!("NavTile {}/{} not found", x, y);
            }
        }
    }
}
