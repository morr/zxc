use super::*;

// #[derive(Component)]
// pub struct FarmTextUI {}

pub struct UiFarmPlugin;

impl Plugin for UiFarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(AppState::Loading),
            render_farm_ui.after(render_selectable_container),
        )
        .add_systems(
            FixedUpdate,
            update_farm_text.run_if(in_state(AppState::Playing)),
        );
    }
}

fn render_farm_ui() {}

fn update_farm_text() {}
