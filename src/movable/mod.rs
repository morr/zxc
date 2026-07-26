use crate::prelude::*;
mod components;
mod systems;
pub use self::components::*;
use self::systems::{move_moving_entities, on_pawn_death_stop_movement};

pub struct MovablePlugin;

impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_pawn_death_stop_movement.run_if(in_state(AppState::Playing)))
            .add_systems(
                Update,
                move_moving_entities.run_if(in_state(AppState::Playing)),
            );
    }
}
