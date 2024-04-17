use super::*;

pub const BASE_WIDTH: i32 = 8;
pub const BASE_HEIGHT: i32 = 14;
pub const FARM_TILE_SIZE: i32 = 1;
pub const HOUSE_SIZE: i32 = 3;

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
            Name::new("Warehouse"),
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
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let size = IVec2::new(FARM_TILE_SIZE, FARM_TILE_SIZE);
    let grid_tile_start = IVec2::new(-13, 0);

    for x in 0..8 {
        for y in 0..5 {
            let grid_tile = IVec2::new(
                grid_tile_start.x + size.x * x,
                grid_tile_start.y + size.y * y,
            );

            commands.spawn((
                FarmTile {},
                Name::new("FarmTile"),
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

            arc_navmesh.write().update_cost(
                grid_tile.x..grid_tile.x + size.x,
                grid_tile.y..grid_tile.y + size.y,
                Some((3.0 * COST_MULTIPLIER) as i32),
            );
        }
    }
}

pub fn spawn_house(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let size = IVec2::new(HOUSE_SIZE, HOUSE_SIZE);
    let grid_tile = IVec2::new(-13, -8);

    commands
        .spawn((
            House {},
            Name::new("House"),
            SpriteBundle {
                texture: assets.house_3.clone(),
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
