use crate::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Navmesh>()
            .add_event::<PathfindRequestEvent>()
            .add_event::<PathfindAnswerEvent>()
            .add_systems(
                Update,
                (
                    listen_for_pathfinding_requests,
                    pathfinding_on_click,
                    listen_for_pathfinding_answers,
                )
                    .chain()
                    .run_if(in_state(WorldState::Playing)),
            );
    }
}
