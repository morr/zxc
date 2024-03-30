use crate::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickTileEvent>()
            .add_event::<HoverTileEvent>()
            .add_systems(OnExit(WorldState::Loading), spawn_map)
            .add_systems(
                Update,
                highlight_hovered_tile.run_if(in_state(WorldState::Playing)),
            );
    }
}
