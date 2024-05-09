use crate::*;

expose_submodules!(
    components,
    systems,
    debug_grid,
    debug_movepath,
    debug_navmesh,
    kayak_ui_v2
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((KayakContextPlugin, KayakWidgets))
            .add_plugins((
                debug_grid::DebugGridPlugin,
                debug_navmesh::DebugNavmeshPlugin,
                debug_movepath::DebugMovepathPlugin,
            ))
            .add_systems(OnExit(AppState::Loading), (render_ui, setup_kayak_ui))
            .add_systems(
                Update,
                (update_ui, handle_ui_keys)//, update_kayak_ui)
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
