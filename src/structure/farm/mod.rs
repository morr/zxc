use crate::prelude::*;
mod components;
mod debug_components;
mod systems;
use self::systems::{
    on_farm_progress, on_farm_state_change, on_farm_tended, on_new_day_schedule_farm_tending,
    progress_harvested_timer, progress_planted_and_tending_rest_timers,
};
pub use self::{components::*, debug_components::*};

pub struct FarmPlugin;

impl Plugin for FarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_new_day_schedule_farm_tending.run_if(in_state(AppState::Playing)))
            .add_observer(on_farm_state_change.run_if(in_state(AppState::Playing)))
            .add_observer(on_farm_tended.run_if(in_state(AppState::Playing)))
            .add_observer(on_farm_progress.run_if(in_state(AppState::Playing)))
            .add_systems(
                FixedUpdate,
                (
                    progress_planted_and_tending_rest_timers,
                    progress_harvested_timer,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
