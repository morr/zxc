use super::*;

// #[derive(Component)]
// pub struct FarmTextUI {}

pub struct UiFarmPlugin;

impl Plugin for UiFarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_farm_ui)
            .add_systems(
                FixedUpdate,
                update_farm_text.run_if(in_state(AppState::Playing)),
            );
    }
}

pub fn render_farm_ui() {}

pub fn update_farm_text() {}
