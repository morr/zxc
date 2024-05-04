use crate::*;

expose_submodules!(components, systems);

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FoodItem>()
            .add_event::<SpawnItemEvent>();
    }
}
