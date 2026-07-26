use crate::prelude::*;
mod components;
mod systems;
pub use self::components::*;
pub use self::systems::progress_pawn_dying;
use self::systems::{
    on_new_day_age_pawns, on_pawn_death, on_pawn_entity_state_change, spawn_pawns,
};
use crate::structure::spawn_base;

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_pawn_death.run_if(in_state(AppState::Playing)))
            .add_observer(on_new_day_age_pawns.run_if(in_state(AppState::Playing)))
            .add_observer(on_pawn_entity_state_change.run_if(in_state(AppState::Playing)))
            .add_systems(OnExit(AppState::Loading), spawn_pawns.after(spawn_base))
            .add_systems(
                FixedUpdate,
                (
                    // update_pawn_color,
                    progress_pawn_dying,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
