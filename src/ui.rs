use crate::settings::Settings;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, render_ui);
    }
}

fn render_ui(mut commands: Commands, settings: Res<Settings>, asset_server: Res<AssetServer>) {
    let time_scale = settings.time_scale;

    commands.spawn(TextBundle::from_section(
        format!("Speed: {time_scale}x"),
        TextStyle {
            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
            font_size: 24.,
            color: Color::WHITE,
        },
    ));
}
