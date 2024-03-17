use bevy::prelude::*;

pub mod components;
mod systems;

pub use components::*;
pub use systems::*;

use crate::structure::spawn_base;
use crate::*;

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pawns.after(spawn_base))
            .add_systems(
                FixedUpdate,
                wander_pawns.run_if(in_state(TimeState::Running)),
            );
    }
}
