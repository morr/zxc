use crate::*;

expose_submodules!(
    components,
    systems,
    debug_grid,
    debug_movepath,
    debug_navmesh,
    debug_info
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug_grid::DebugGridPlugin)
            .add_plugins(debug_navmesh::DebugNavmeshPlugin)
            .add_plugins(debug_movepath::DebugMovepathPlugin)
            .add_systems(
                OnExit(AppState::Loading),
                (render_simulation_ui, render_items_stock_ui, render_debug_info),
            )
            .add_systems(
                Update,
                (
                    update_simulation_speed_text,
                    update_simulation_date_time_text,
                    update_food_stock_text,
                    update_debug_info,
                    handle_debug_info_keys,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
