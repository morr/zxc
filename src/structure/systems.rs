use super::*;

pub fn spawn_base(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let structure_size = IVec2::new(BASE_WIDTH, BASE_HEIGHT);
    let structure_grid_pos = IVec2::new(0, 0);

    commands
        .spawn((
            Structure {},
            Name::new("Base"),
            SpriteBundle {
                texture: assets.castle.clone(),
                sprite: Sprite {
                    custom_size: Some(structure_size.grid_tile_edge_to_world()),
                    ..default()
                },
                transform: Transform::from_translation(
                    (structure_grid_pos.grid_tile_edge_to_world()
                        + structure_size.grid_tile_edge_to_world() / 2.0)
                        .extend(STRUCTURE_Z_INDEX),
                ),
                ..default()
            },
        ))
        .insert(ShowAabbGizmo {
            color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
        });

    arc_navmesh.write().update_cost(
        (structure_grid_pos.x)..(structure_grid_pos.x + structure_size.x),
        (structure_grid_pos.y)..(structure_grid_pos.x + structure_size.y),
        None,
    )
}
