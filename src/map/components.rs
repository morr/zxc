use bevy::prelude::*;
use bevy_flowfield_tiles_plugin::flowfields::{
    fields::{FieldCell, RouteMetadata},
    sectors::SectorID,
};

#[derive(Component, Debug)]
pub struct Tile(pub Vec2);

#[derive(Component)]
pub struct TileHovered;

#[derive(Event, Debug)]
pub struct HoverTileEvent(pub Vec2);

#[derive(Event, Debug)]
pub struct ClickTileEvent(pub Vec2);

#[derive(Component)]
pub struct Actor;

#[derive(Default, Component, Debug)]
pub struct Pathing {
    pub target_position: Option<Vec2>,
    pub metadata: Option<RouteMetadata>,
    pub portal_route: Option<Vec<(SectorID, FieldCell)>>,
    pub has_los: bool,
}
