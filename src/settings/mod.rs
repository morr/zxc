use bevy::prelude::*;

#[derive(Resource)]
pub struct Settings {
    pub time_scale: f32,
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Settings { time_scale: 1.0 })
            .add_systems(FixedUpdate, update_settings);
    }
}

fn update_settings(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Settings>) {
    if keys.just_pressed(KeyCode::Equal) {
        settings.time_scale += 1.0;
    }
    if keys.just_pressed(KeyCode::Minus) {
        settings.time_scale -= 1.0;
    }
}
