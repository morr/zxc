use crate::{ui::UpdateUiEvent, TimeState};
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
    time_state: Res<State<TimeState>>,
    mut next_state: ResMut<NextState<TimeState>>,
    mut settings: ResMut<Settings>,
    mut ev_update_ui: EventWriter<UpdateUiEvent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        match time_state.get() {
            TimeState::Running => next_state.set(TimeState::Paused),
            TimeState::Paused => next_state.set(TimeState::Running),
        };
        ev_update_ui.send(UpdateUiEvent {});
    }

    if keys.just_pressed(KeyCode::Equal) {
        match time_state.get() {
            TimeState::Running => settings.time_scale += 1.0,
            TimeState::Paused => next_state.set(TimeState::Running),
        };
        ev_update_ui.send(UpdateUiEvent {});
    }
    if keys.just_pressed(KeyCode::Minus) {
        if let TimeState::Running = time_state.get() {
            if settings.time_scale == 1.0 {
                next_state.set(TimeState::Paused);
            } else {
                settings.time_scale -= 1.0;
            }
            ev_update_ui.send(UpdateUiEvent {});
        }
    }
}
