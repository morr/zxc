use bevy::prelude::*;

use super::*;
use crate::{utils::grid_tile_edge_to_world, GRID_COLS_HALF, GRID_ROWS_HALF, TILE_SIZE, TILE_Z_INDEX};

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // https://itch.io/game-assets/free/tag-textures
    // https://screamingbrainstudios.itch.io/tiny-texture-pack/download/eyJpZCI6MTAzMzEyOSwiZXhwaXJlcyI6MTcxMDc5ODI3OX0%3d.%2f%2bodleBeo8EbYeM%2bKnn3UZPKq2U%3d
    let texture_handle = asset_server.load("sprites/grass.png");

    for x in (-1 * GRID_COLS_HALF)..GRID_COLS_HALF {
        for y in (-1 * GRID_ROWS_HALF)..GRID_ROWS_HALF {
            commands
                .spawn(SpriteBundle {
                    texture: texture_handle.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        grid_tile_edge_to_world(x) + TILE_SIZE / 2.,
                        grid_tile_edge_to_world(y) + TILE_SIZE / 2.,
                        TILE_Z_INDEX,
                    ),
                    ..default()
                })
                .insert(Tile(IVec2::new(x, y)));
        }
    }
}

pub fn highlight_hovered_tile(
    mut commands: Commands,
    mut event_reader: EventReader<HoverTileEvent>,
    query_tiles_hovered: Query<Entity, With<TileHovered>>,
    query_tiles: Query<(&Tile, Entity)>,
) {
    for event in event_reader.read() {
        remove_tile_hovered_from_other_tiles(&query_tiles_hovered, &mut commands);

        for (tile, entity) in query_tiles.iter() {
            if tile.0 == event.0 {
                commands
                    .entity(entity)
                    .insert(TileHovered {})
                    .insert(ShowAabbGizmo {
                        color: Some(Color::WHITE),
                    });
            }
        }
    }
}

fn remove_tile_hovered_from_other_tiles(
    query: &Query<Entity, With<TileHovered>>,
    commands: &mut Commands,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .remove::<TileHovered>()
            .remove::<ShowAabbGizmo>();
    }
}
