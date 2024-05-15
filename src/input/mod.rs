use crate::*;

expose_submodules!(components, systems);

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveredGridTile>()
            .add_event::<HoverEvent>()
            .add_event::<ClickEvent>()
            .add_systems(Update, mouse_input.run_if(in_state(AppState::Playing)));
    }
}
