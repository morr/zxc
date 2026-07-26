use crate::prelude::*;
mod components;
mod systems;
pub use self::components::*;
use self::systems::{
    on_merge_carryable, on_spawn_carryable, on_store_carryable, spawn_initial_items,
};

pub struct CarryablePlugin;

impl Plugin for CarryablePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FoodStock>()
            .add_observer(on_spawn_carryable.run_if(in_state(AppState::Playing)))
            .add_observer(on_store_carryable.run_if(in_state(AppState::Playing)))
            .add_observer(on_merge_carryable.run_if(in_state(AppState::Playing)))
            .add_systems(OnExit(AppState::Loading), spawn_initial_items);
    }
}
