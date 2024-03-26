use crate::prelude::*;

pub mod components;
pub use components::*;

mod debug_grid;
mod debug_movepath;
mod debug_navmesh;

mod systems;
pub use systems::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug_grid::DebugGridPlugin)
            .add_plugins(debug_navmesh::DebugNavmeshPlugin)
            .add_plugins(debug_movepath::DebugMovepathPlugin)
            // .add_event::<UpdateUiEvent>()
            .add_systems(Startup, render_ui)
            .add_systems(Update, (update_ui, handle_ui_keys));
    }
}
