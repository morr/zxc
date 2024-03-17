use crate::settings::Settings;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateUiEvent>()
            .add_systems(Startup, render_ui)
            .add_systems(FixedUpdate, update_ui);
    }
}

#[derive(Event)]
pub struct UpdateUiEvent {}

#[derive(Component)]
pub struct DebugLine {}

fn render_ui(mut commands: Commands, settings: Res<Settings>, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            speed_line(&settings),
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 24.,
                color: Color::WHITE,
            },
        ),
        DebugLine {},
    ));
}

fn update_ui(
    settings: Res<Settings>,
    mut ev_update_ui: EventReader<UpdateUiEvent>,
    mut q: Query<&mut Text, With<DebugLine>>,
) {
    for _ev in ev_update_ui.read() {
        println!("update ui");

        let mut text = q.single_mut();
        text.sections[0].value = speed_line(&settings);
    }
}

fn speed_line(settings: &Res<Settings>) -> String {
    format!("Speed: {}x", settings.time_scale)
}
