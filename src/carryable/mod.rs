use crate::*;

expose_submodules!(components, systems);

pub struct CarryablePlugin;

impl Plugin for CarryablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Carryable>()
            .init_resource::<FoodStock>()
            .add_event::<SpawnCarryableEvent>()
            .add_systems(OnExit(AppState::Loading), spawn_initial_items)
            .add_systems(
                FixedUpdate,
                spawn_on_event.run_if(in_state(AppState::Playing)),
            );
    }
}
