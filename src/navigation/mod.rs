use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        // app.
        // .add_systems(
        //     Startup,
        //     setup_navigation::setup_navigation.after(spawn_map::spawn_map),
        // )
        // .add_systems(
        //     Update,
        //     (
        //         setup_navigation::pathfinding_on_click,
        //         setup_navigation::get_or_request_route,
        //         setup_navigation::actor_steering,
        //     ,
        // )
        // .add_systems(Update, highlight_hovered_tile);
    }
}
