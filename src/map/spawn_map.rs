use bevy::prelude::*;

use super::*;
use crate::{utils::tile_pos_to_world, GRID_COLS_HALF, GRID_ROWS_HALF, TILE_SIZE, TILE_Z_INDEX};

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // https://itch.io/game-assets/free/tag-textures
    // https://screamingbrainstudios.itch.io/tiny-texture-pack/download/eyJpZCI6MTAzMzEyOSwiZXhwaXJlcyI6MTcxMDc5ODI3OX0%3d.%2f%2bodleBeo8EbYeM%2bKnn3UZPKq2U%3d
    let texture_handle = asset_server.load("sprites/grass.png");

    for i_x in (-1 * GRID_COLS_HALF as i32)..(GRID_COLS_HALF as i32) {
        for i_y in (-1 * GRID_ROWS_HALF as i32)..(GRID_ROWS_HALF as i32) {
            let x = i_x as f32;
            let y = i_y as f32;

            commands
                .spawn(SpriteBundle {
                    texture: texture_handle.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        tile_pos_to_world(x) + TILE_SIZE / 2.,
                        tile_pos_to_world(y) + TILE_SIZE / 2.,
                        TILE_Z_INDEX,
                    ),
                    ..default()
                })
                .insert(Tile(Vec2::new(x, y)));
        }
    }
}
