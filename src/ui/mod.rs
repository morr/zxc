use crate::*;

expose_submodules!(components, systems, items_stock, pawn, farm, debug);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiDebugPlugin,
            UiPawnPlugin,
            UiFarmPlugin,
        ))
        .add_systems(OnExit(AppState::Loading), render_simulation_speed_ui)
        .add_systems(
            FixedUpdate,
            (
                // update_simulation_season_text,
                update_simulation_speed_text,
                update_simulation_date_time_text,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            Update,
            (handle_debug_info_keys,)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}
