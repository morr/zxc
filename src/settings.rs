use crate::ui::UpdateUiEvent;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Settings {
    pub time_scale: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { time_scale: 1.0 }
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>()
            .add_systems(FixedUpdate, update_settings);
    }
}

fn update_settings(
    mut settings: ResMut<Settings>,
    mut ev_update_ui: EventWriter<UpdateUiEvent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Equal) {
        settings.time_scale += 1.0;
        ev_update_ui.send(UpdateUiEvent {});
    }
    if keys.just_pressed(KeyCode::Minus) {
        settings.time_scale -= 1.0;
        ev_update_ui.send(UpdateUiEvent {});
    }
}
