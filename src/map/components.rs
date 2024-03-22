use bevy::prelude::*;

#[derive(Component)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct TileHovered;

#[derive(Event)]
pub struct HoverTileEvent {
    pub x: u32,
    pub y: u32,
}
