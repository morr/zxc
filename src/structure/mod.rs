use crate::prelude::*;

pub mod components;
pub mod systems;

pub use components::*;
pub use systems::*;

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_base);
    }
}
