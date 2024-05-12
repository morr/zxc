use crate::*;

expose_submodules!(grid, movepath, navmesh, info);

#[derive(Component)]
struct DebugStatusTextUIMarker {}

#[derive(Component)]
struct DebugHelpBlockUIMarker {}

pub struct UiDebugPlugin;

impl Plugin for UiDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DebugGridPlugin)
            .add_plugins(DebugNavmeshPlugin)
            .add_plugins(DebugMovepathPlugin)
            .add_systems(OnExit(AppState::Loading), render_debug_info)
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
