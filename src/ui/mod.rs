use crate::*;
expose_submodules!(
    components,
    systems,
    debug_grid,
    debug_movepath,
    debug_navmesh
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug_grid::DebugGridPlugin)
            .add_plugins(debug_navmesh::DebugNavmeshPlugin)
            .add_plugins(debug_movepath::DebugMovepathPlugin)
            // .add_event::<UpdateUiEvent>()
            .add_systems(OnExit(AppState::Loading), render_ui)
            .add_systems(
                Update,
                (update_ui, handle_ui_keys)
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
