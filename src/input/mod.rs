use crate::*;

expose_submodules!(components, systems);

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PrevHoveredTilePos>()
            .add_systems(Update, mouse_input);
    }
}
