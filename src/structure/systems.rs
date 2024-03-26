use bevy::prelude::*;

use super::*;
use crate::{
    navigation::components::NavMesh,
    utils::GridTranslationHelper,
    STRUCTURE_Z_INDEX, TILE_SIZE,
};

pub fn spawn_base(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut navmesh: ResMut<NavMesh>,
) {
    // println!("Spawning base");

    // let mesh = Mesh::from(Rectangle::new(
    //     BASE_WIDTH * TILE_SIZE,
    //     BASE_HEIGHT * TILE_SIZE,
    // ));
    // let material = ColorMaterial::from(Color::rgb(1., 0., 0.));

    // let mesh_handle = meshes.add(mesh);
    // let material_handle = materials.add(material);
    // https://fin-nio.itch.io/pixel-houses
    let texture_handle = asset_server.load("sprites/castle_complete.png");
    let grid_pos = IVec2::new(2, 0);

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
            // mesh_bundle: MaterialMesh2dBundle {
            //     mesh: mesh_handle.into(),
            //     material: material_handle,
            //     transform: Transform::from_xyz(
            //         tile_pos_to_world(GRID_COLS as f32 / 2.0),
            //         tile_pos_to_world(GRID_ROWS as f32 / 2.0),
            //         STRUCTURE_Z_INDEX,
            //     ),
            //     ..default()
            // },
        })
        .insert(ShowAabbGizmo {
            color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
        });

    // mark navmesh tiles as occupied
    for x in (grid_pos.x - (BASE_WIDTH / 2.0) as i32)..(grid_pos.x + (BASE_WIDTH / 2.0) as i32) {
        for y in
            (grid_pos.y - (BASE_HEIGHT / 2.0) as i32)..(grid_pos.x + (BASE_HEIGHT / 2.0) as i32)
        {
            if let Some(navtile) = navmesh.get_mut(x, y) {
                navtile.passable = false
            } else {
                error!("NavTile {}/{} not found", x, y);
            }
        }
    }
}
