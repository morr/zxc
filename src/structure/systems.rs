use super::*;

pub const BASE_WIDTH: i32 = 8;
pub const BASE_HEIGHT: i32 = 14;
pub const FARM_TILE_SIZE: i32 = 1;
pub const HOUSE_WIDTH: i32 = 4;
pub const HOUSE_HEIGHT: i32 = 4;
pub const WELL_SIZE: i32 = 2;
pub const BED_SIZE: i32 = 1;

pub fn spawn_base(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let size = IVec2::new(BASE_WIDTH, BASE_HEIGHT);
    let grid_tile = IVec2::new(0, 0);

    commands.spawn((
        Warehouse::default(),
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
    ));
    // .insert(ShowAabbGizmo {
    //     color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
    // });

    arc_navmesh.write().update_cost(
        (grid_tile.x)..(grid_tile.x + size.x),
        (grid_tile.y)..(grid_tile.y + size.y),
        None,
    )
}

pub fn spawn_farm(
    mut commands: Commands,
    assets: Res<FarmAssets>,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut state_change_event_writer: EventWriter<EntityStateChangeEvent<FarmState>>,
) {
    let grid_tile_start = IVec2::new(-13, 0);
    let mut navmesh = arc_navmesh.write();

    for x in 0..CONFIG.starting_scene.farm_width {
        for y in 0..CONFIG.starting_scene.farm_height {
            let grid_tile = IVec2::new(
                grid_tile_start.x + FARM_TILE_SIZE * x,
                grid_tile_start.y + FARM_TILE_SIZE * y,
            );

            Farm::spawn(
                &mut commands,
                &assets,
                &mut navmesh,
                grid_tile,
                &mut state_change_event_writer,
            );
        }
    }
}

pub fn spawn_house(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let size = IVec2::new(HOUSE_WIDTH, HOUSE_HEIGHT);
    let grid_tile = IVec2::new(-13, -8);

    commands.spawn((
        House::default(),
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
    ));
    // .insert(ShowAabbGizmo {
    //     color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
    // });

    arc_navmesh.write().update_cost(
        (grid_tile.x)..(grid_tile.x + size.x),
        (grid_tile.y)..(grid_tile.y + size.y),
        None,
    )
}

pub fn spawn_well(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let size = IVec2::new(WELL_SIZE, WELL_SIZE);
    let grid_tile = IVec2::new(-3, -6);

    commands.spawn((
        Well::default(),
        Name::new("well"),
        SpriteBundle {
            texture: assets.well.clone(),
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
    ));
    // .insert(ShowAabbGizmo {
    //     color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
    // });

    arc_navmesh.write().update_cost(
        (grid_tile.x)..(grid_tile.x + size.x),
        (grid_tile.y)..(grid_tile.y + size.y),
        None,
    )
}

pub fn spawn_bed(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let size = IVec2::new(BED_SIZE, BED_SIZE);

    for x in 0..CONFIG.starting_scene.beds_num {
        let grid_tile = IVec2::new(-7 + x * BED_SIZE, 3);

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
            //     color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
            // })
            .id();

        let mut navmesh = arc_navmesh.write();
        navmesh.update_cost(
            (grid_tile.x)..(grid_tile.x + size.x),
            (grid_tile.y)..(grid_tile.y + size.y),
            Navtile::config_cost_to_pathfinding_cost(CONFIG.movement_cost.furniture),
        );
        navmesh.add_occupation::<Bed>(id, grid_tile.x, grid_tile.y);
    }
}
