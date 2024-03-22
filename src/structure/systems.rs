use bevy::prelude::*;

use super::*;
use crate::{utils::tile_pos_to_world, GRID_COLS, GRID_ROWS, STRUCTURE_Z_INDEX, TILE_SIZE};

pub fn spawn_base(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning base");

    // let mesh = Mesh::from(Rectangle::new(
    //     BASE_WIDTH * TILE_SIZE,
    //     BASE_HEIGHT * TILE_SIZE,
    // ));
    // let material = ColorMaterial::from(Color::rgb(1., 0., 0.));

    // let mesh_handle = meshes.add(mesh);
    // let material_handle = materials.add(material);
    // https://fin-nio.itch.io/pixel-houses
    let texture_handle = asset_server.load("sprites/castle_complete.png");

    commands.spawn((
        StructureBundle {
            structure: Structure {},
            name: Name::new("Base"),
            sprite_bundle: SpriteBundle {
                texture: texture_handle.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(BASE_WIDTH * TILE_SIZE, BASE_HEIGHT * TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    tile_pos_to_world((GRID_COLS as f32 / 2.0) as u32),
                    tile_pos_to_world((GRID_ROWS as f32 / 2.0) as u32),
                    STRUCTURE_Z_INDEX,
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
        },
        ShowAabbGizmo {
            color: Some(Color::WHITE),
        },
    ));
}
