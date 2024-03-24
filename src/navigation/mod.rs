use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

use crate::map::systems::spawn_map;

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PathfindingRequestEvent>()
            .add_systems(Startup, generate_navmesh.after(spawn_map))
            .add_systems(
                Update,
                (
                    listen_for_pathfinding_requests,
                    pathfinding_on_click, //         setup_navigation::get_or_request_route,
                                          //         setup_navigation::actor_steering,
                                          //     ,
                ),
            );
        // .add_systems(Update, highlight_hovered_tile);
    }
}