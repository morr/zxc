use crate::*;

expose_submodules!(components, systems, utils);

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), generate_map)
            .add_systems(Update, track_hover.run_if(in_state(AppState::Playing)));
    }
}
