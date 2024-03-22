use bevy::prelude::*;

pub mod components;
use components::*;

use crate::{utils::tile_pos_to_world, GRID_COLS, GRID_ROWS, TILE_SIZE, TILE_Z_INDEX};

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // https://itch.io/game-assets/free/tag-textures
    // https://screamingbrainstudios.itch.io/tiny-texture-pack/download/eyJpZCI6MTAzMzEyOSwiZXhwaXJlcyI6MTcxMDc5ODI3OX0%3d.%2f%2bodleBeo8EbYeM%2bKnn3UZPKq2U%3d
    let texture_handle = asset_server.load("sprites/grass.png");

    for x in 0..GRID_COLS {
        for y in 0..GRID_ROWS {
            commands.spawn((
                SpriteBundle {
                    texture: texture_handle.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        tile_pos_to_world(x),
                        tile_pos_to_world(y),
                        TILE_Z_INDEX,
                    ),
                    ..default()
                },
                TileComponent { x, y },
            ));
        }
    }
}
