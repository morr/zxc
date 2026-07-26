use crate::prelude::*;
mod bed;
mod components;
mod farm;
mod storage;
mod systems;
pub use self::systems::{
    BASE_HEIGHT, BASE_WIDTH, BED_SIZE, FARM_TILE_SIZE, STORAGE_SIZE, spawn_base,
};
use self::systems::{spawn_bed, spawn_farm, spawn_storage};
pub use self::{bed::*, components::*, farm::*, storage::*};

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BedPlugin, FarmPlugin)).add_systems(
            OnExit(AppState::Loading),
            (
                // spawn_base,
                spawn_farm,
                // spawn_house,
                // spawn_well,
                spawn_bed,
                spawn_storage,
            ),
        );
    }
}
