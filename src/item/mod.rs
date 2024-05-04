use crate::*;

expose_submodules!(components, systems);

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FoodItem>()
            .add_event::<SpawnItemEvent>()
            .add_systems(
                FixedUpdate,
                spawn_item_on_event.run_if(in_state(AppState::Playing)),
            );
    }
}
