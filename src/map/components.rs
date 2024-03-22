use bevy::prelude::*;

#[derive(Component)]
pub struct TileComponent {
    pub x: u32,
    pub y: u32,
}

pub struct TileHovered;

#[derive(Event)]
pub struct HoverTileEvent {
    pub x: u32,
    pub y: u32,
}
