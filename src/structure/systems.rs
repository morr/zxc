use super::*;

pub fn spawn_base(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let size = IVec2::new(BASE_WIDTH, BASE_HEIGHT);
    let grid_tile = IVec2::new(0, 0);

    commands
        .spawn((
            Warehouse {},
            SpriteBundle {
                texture: assets.castle.clone(),
                sprite: Sprite {
                    custom_size: Some(size.grid_tile_edge_to_world()),
                    ..default()
                },
                transform: Transform::from_translation(
                    (grid_tile.grid_tile_edge_to_world() + size.grid_tile_edge_to_world() / 2.0)
                        .extend(STRUCTURE_Z_INDEX),
                ),
                ..default()
            },
        ))
        .insert(ShowAabbGizmo {
            color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
        });

    arc_navmesh.write().update_cost(
        (grid_tile.x)..(grid_tile.x + size.x),
        (grid_tile.y)..(grid_tile.x + size.y),
        None,
    )
}

pub fn spawn_farm(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    // arc_navmesh: ResMut<ArcNavmesh>,
) {
    let size = IVec2::new(1, 1);
    let grid_tile_start = IVec2::new(-13, 0);

    for x in 0..8 {
        for y in 0..5 {
            let grid_tile = IVec2::new(
                grid_tile_start.x + size.x * x,
                grid_tile_start.y + size.y * y,
            );

            commands
                .spawn((
                    FarmTile {},
                    SpriteBundle {
                        texture: assets.dirt.clone(),
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
                ));
        }
    }

    // arc_navmesh.write().update_cost(
    //     (structure_grid_pos.x)..(structure_grid_pos.x + structure_size.x),
    //     (structure_grid_pos.y)..(structure_grid_pos.x + structure_size.y),
    //     None,
    // )
}
