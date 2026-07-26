use crate::*;

mod components;
mod systems;
pub use self::{components::*, systems::*};

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
