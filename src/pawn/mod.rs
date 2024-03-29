use crate::prelude::*;

pub mod components;
pub mod systems;

pub use components::*;
pub use systems::*;

use crate::structure::spawn_base;
use crate::*;

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EntityStateChangeEvent<MovementState>>()
            .add_systems(OnExit(WorldState::Loading), spawn_pawns.after(spawn_base))
            .add_systems(
                FixedUpdate,
                update_pawn_color.run_if(in_state(WorldState::Playing)),
            );
        // .add_systems(
        //     FixedUpdate,
        //     wander_pawns.run_if(in_state(TimeState::Running)),
        // );
    }
}
