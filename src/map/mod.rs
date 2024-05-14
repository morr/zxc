use crate::*;

expose_submodules!(components, systems, utils);

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickTileEvent>()
            .add_event::<TileHoveredEvent>()
            .add_systems(OnExit(AppState::Loading), spawn_map);
            // .add_systems(
            //     Update,
            //     highlight_hovered_tile.run_if(in_state(AppState::Playing)),
            // );
    }
}
