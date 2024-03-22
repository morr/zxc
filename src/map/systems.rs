use bevy::prelude::*;

use super::*;

pub fn highlight_hovered_tile(
    mut commands: Commands,
    mut event_reader: EventReader<HoverTileEvent>,
    query_tiles_hovered: Query<Entity, With<TileHovered>>,
    query_tiles: Query<(&Tile, Entity)>,
) {
    for event in event_reader.read() {
        remove_tile_hovered_from_other_tiles(&query_tiles_hovered, &mut commands);

        for (tile, entity) in query_tiles.iter() {
            if tile.x == event.x && tile.y == event.y {
                // println!("hovered {:?}", tile);

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
