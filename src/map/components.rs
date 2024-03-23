use bevy::prelude::*;
use bevy_flowfield_tiles_plugin::flowfields::{
    fields::{FieldCell, RouteMetadata},
    sectors::SectorID,
};

#[derive(Component, Debug)]
pub struct Tile(pub UVec2);

#[derive(Component)]
pub struct TileHovered;

#[derive(Event, Debug)]
pub struct HoverTileEvent(pub UVec2);

#[derive(Event, Debug)]
pub struct ClickTileEvent(pub UVec2);

#[derive(Component)]
pub struct Actor;

#[derive(Default, Component)]
pub struct Pathing {
    pub target_position: Option<UVec2>,
    pub metadata: Option<RouteMetadata>,
    pub portal_route: Option<Vec<(SectorID, FieldCell)>>,
    pub has_los: bool,
}
