use crate::*;

expose_submodules!(components, systems);

pub struct CarryablePlugin;

impl Plugin for CarryablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Carryable>()
            .init_resource::<FoodStock>()
            .add_observer(on_spawn_carryable)
            .add_observer(on_store_carryable)
            .add_observer(on_merge_carryable)
            .add_systems(OnExit(AppState::Loading), spawn_initial_items);
    }
}
