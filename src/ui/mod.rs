use bevy::prelude::*;

pub mod components;
// pub use components::*;

mod debug_grid;

mod systems;
pub use systems::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug_grid::DebugGridPlugin)
            // .add_event::<UpdateUiEvent>()
            .add_systems(Startup, render_ui)
            .add_systems(FixedUpdate, update_ui)
            .add_systems(FixedUpdate, handle_ui_keys);
    }
}
