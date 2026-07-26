use crate::prelude::*;
mod components;
pub mod generator;
mod systems;
mod tileset;
mod utils;
pub use self::{components::*, systems::*, tileset::*, utils::*};

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(config().map_generator.clone())
            .add_observer(on_rebuild_map)
            .add_systems(
                OnExit(AppState::Loading),
                (build_tileset, generate_map).chain(),
            )
            .add_plugins(generator::perlin_noise::PerlinNoisePlugin);
    }
}
