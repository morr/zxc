use bevy::prelude::*;

pub mod components;
mod systems;

pub use components::*;
pub use systems::*;

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pawns);
    }
}
