use crate::*;

expose_submodules!(grid, movepath, navmesh, info, tasks_queue);

#[derive(Component, Default)]
pub struct DebugUiContainerarker {}

#[derive(Component, Default)]
pub struct DebugStatusTextUIMarker {}

#[derive(Component, Default)]
pub struct DebugHelpBlockUIMarker {}

pub struct UiDebugPlugin;

impl Plugin for UiDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DebugGridPlugin)
            .add_plugins(DebugNavmeshPlugin)
            .add_plugins(DebugMovepathPlugin)
            .add_systems(OnExit(AppState::Loading), render_debug_ui_container)
            .add_systems(
                OnExit(AppState::Loading),
                (render_debug_info, render_tasks_ui)
                    .chain()
                    .after(render_debug_ui_container),
            )
            .add_systems(
                FixedUpdate,
                update_debug_info.run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                Update,
                handle_debug_info_keys.run_if(in_state(AppState::Playing)),
            );
    }
}
