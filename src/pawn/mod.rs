use crate::prelude::*;

pub mod components;
pub mod systems;

pub use components::*;
pub use systems::*;

use crate::structure::spawn_base;
use crate::*;

use self::story_time::TimeState;

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pawns.after(spawn_base))
            .add_systems(Update, move_pawns.run_if(in_state(TimeState::Running)));
        // .add_systems(
        //     FixedUpdate,
        //     wander_pawns.run_if(in_state(TimeState::Running)),
        // );
    }
}
