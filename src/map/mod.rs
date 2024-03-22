use bevy::prelude::*;

pub mod components;
mod spawn_map;
pub mod systems;

use components::*;
use systems::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map::spawn_map)
            .add_systems(Update, highlight_hovered_tile);
    }
}
