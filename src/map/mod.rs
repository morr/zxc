use bevy::prelude::*;

pub mod components;
mod setup_navigation;
mod spawn_map;
pub mod systems;

use bevy_flowfield_tiles_plugin::plugin::FlowFieldTilesPlugin;
use components::*;
use systems::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FlowFieldTilesPlugin)
            .add_event::<ClickTileEvent>()
            .add_event::<HoverTileEvent>()
            .add_systems(Startup, spawn_map::spawn_map)
            .add_systems(
                Startup,
                setup_navigation::setup_navigation.after(spawn_map::spawn_map),
            )
            .add_systems(Update, setup_navigation::pathfinding_on_click)
            .add_systems(Update, highlight_hovered_tile);
    }
}
