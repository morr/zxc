use bevy::prelude::*;

pub mod components;
// pub use components::*;

mod debug_grid;

mod systems;
pub use systems::*;

use crate::map::components::HoverTileEvent;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug_grid::DebugGridPlugin)
            // .add_event::<UpdateUiEvent>()
            .add_event::<HoverTileEvent>()
            .add_systems(Startup, render_ui)
            .add_systems(Update, (update_ui, handle_ui_keys));
    }
}
