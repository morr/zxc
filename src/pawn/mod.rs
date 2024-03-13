use bevy::prelude::*;

pub mod components;
mod systems;

pub use components::*;
pub use systems::*;

use crate::structure::spawn_base;

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pawns.after(spawn_base));
    }
}
