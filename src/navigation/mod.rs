use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

use crate::map::systems::spawn_map;

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Navmesh>()
            .add_event::<PathfindRequestEvent>()
            .add_event::<PathfindAnswerEvent>()
            .add_systems(Startup, generate_navmesh.after(spawn_map))
            .add_systems(
                Update,
                (
                    listen_for_pathfinding_requests,
                    pathfinding_on_click,
                    listen_for_pathfinding_answers,
                ),
            );
    }
}
