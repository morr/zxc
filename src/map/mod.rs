use crate::*;

pub mod generator;
expose_submodules!(components, systems, utils);

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(config().map_generator.clone())
            .register_type::<Tile>()
            .add_observer(on_rebuild_map)
            .add_observer(on_hover)
            .add_systems(OnExit(AppState::Loading), generate_map)
            .add_plugins(generator::perlin_noise::PerlinNoisePlugin);
    }
}
