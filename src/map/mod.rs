use crate::*;

pub mod generator;
expose_submodules!(components, systems, utils);

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tile>()
            .add_event::<RebuildMapEvent>()
            .add_systems(OnExit(AppState::Loading), generate_map)
            .add_systems(Update, track_hover.run_if(in_state(AppState::Playing)));

        #[cfg(feature = "map_generator")]
        app
            .add_plugins((
                generator::cellular_automata::CellularAutomataPlugin,
                generator::markov_junior::MarkovJuniorPlugin,
                generator::perlin_noise::PerlinNoisePlugin
            ))
            .add_systems(FixedUpdate, rebuild_map.run_if(in_state(AppState::Playing)));

    }
}
