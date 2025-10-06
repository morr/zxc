use super::*;

pub const BASE_WIDTH: i32 = 8;
pub const BASE_HEIGHT: i32 = 14;
pub const HOUSE_WIDTH: i32 = 4;
pub const HOUSE_HEIGHT: i32 = 4;
pub const WELL_SIZE: i32 = 2;
pub const FARM_TILE_SIZE: i32 = 1;
pub const BED_SIZE: i32 = 1;
pub const STORAGE_SIZE: i32 = 1;

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
        Sprite {
            image: assets.castle.clone(),
            custom_size: Some(size.grid_tile_edge_to_world()),
            ..default()
        },
        Transform::from_translation(
            (grid_tile.grid_tile_edge_to_world() + size.grid_tile_edge_to_world() / 2.0)
                .extend(STRUCTURE_Z_INDEX),
        ),
    ));
    // .insert(ShowAabbGizmo {
    //     color: Some(Color::srgba(1.0, 1.0, 1.0, 0.25)),
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
    mut state_change_event_writer: MessageWriter<EntityStateChangeMessage<FarmState>>,
) {
    let grid_tile_start = IVec2::new(-13, 0);
    let mut navmesh = arc_navmesh.write();

    let mut farms_spawned = 0;
    let half_size = (config().starting_scene.farms as f32).sqrt().ceil() as i32;

    for x in 0..half_size {
        for y in 0..half_size {
            if farms_spawned == config().starting_scene.farms {
                continue;
            }
            let grid_tile = IVec2::new(
                grid_tile_start.x + FARM_TILE_SIZE * x,
                grid_tile_start.y + FARM_TILE_SIZE * y,
            );

            Farm::spawn(
                grid_tile,
                &mut commands,
                &assets,
                &mut navmesh,
                &mut state_change_event_writer,
            );
            farms_spawned += 1;
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
        Sprite {
            image: assets.house_3.clone(),
            custom_size: Some(size.grid_tile_edge_to_world()),
            ..default()
        },
        Transform::from_translation(
            (grid_tile.grid_tile_edge_to_world() + size.grid_tile_edge_to_world() / 2.0)
                .extend(STRUCTURE_Z_INDEX),
        ),
    ));
    // .insert(ShowAabbGizmo {
    //     color: Some(Color::srgba(1.0, 1.0, 1.0, 0.25)),
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
        Sprite {
            image: assets.well.clone(),
            custom_size: Some(size.grid_tile_edge_to_world()),
            ..default()
        },
        Transform::from_translation(
            (grid_tile.grid_tile_edge_to_world() + size.grid_tile_edge_to_world() / 2.0)
                .extend(STRUCTURE_Z_INDEX),
        ),
    ));
    // .insert(ShowAabbGizmo {
    //     color: Some(Color::srgba(1.0, 1.0, 1.0, 0.25)),
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
    mut available_beds: ResMut<AvailableBeds>,
) {
    let mut navmesh = arc_navmesh.write();

    for x in 0..config().starting_scene.beds {
        let grid_tile = IVec2::new(-7 + x * BED_SIZE, 3);

        Bed::spawn(
            grid_tile,
            &mut commands,
            assets.bed.clone(),
            &mut navmesh,
            &mut available_beds,
        );
    }
}

pub fn spawn_storage(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let mut navmesh = arc_navmesh.write();

    for x in 0..config().starting_scene.storages {
        let grid_tile = IVec2::new(-15 + x, 6);

        Storage::spawn(
            grid_tile,
            &mut commands,
            assets.storage.clone(),
            &mut navmesh,
        );
    }
}
