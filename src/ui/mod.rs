use crate::*;

expose_submodules!(
    components,
    systems,
    pawn_info,
    debug
);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiDebugPlugin)
            .add_systems(
                OnExit(AppState::Loading),
                (
                    // render_simulation_season_ui,
                    render_simulation_speed_ui,
                    render_items_stock_ui,
                    render_pawn_ui,
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    // update_simulation_season_text,
                    update_simulation_speed_text,
                    update_simulation_date_time_text,
                    update_pawn_age_text,
                    update_pawn_lifetime_text,
                    update_pawn_birthday_text,
                    update_food_stock_text,
                    update_pawn_stock_text,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                Update,
                (
                    handle_debug_info_keys,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
