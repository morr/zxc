use bevy::prelude::*;

use super::*;

pub fn highlight_hovered_tile(
    mut commands: Commands,
    mut event_reader: EventReader<HoverTileEvent>,
    query: Query<(&Tile, Entity), With<TileHovered>>,
) {
    for event in event_reader.read() {
        for (tile, entity) in query.iter() {
            // remove highlight from not hovered tiles
            if tile.x != event.x || tile.y != event.y {
                commands.entity(entity).remove::<TileHovered>();
                println!("remove TileHovered from {}x{}", tile.x, tile.y);
            }
        }
        // println!("{}/{}", event.x, event.y);
    }
}
