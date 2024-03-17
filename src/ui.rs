use crate::{settings::Settings, TimeState};
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

fn render_ui(
    mut commands: Commands,
    settings: Res<Settings>,
    time_state: Res<State<TimeState>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands.spawn((
        TextBundle::from_section(
            format_ui_line(&settings, &time_state),
            TextStyle {
                font: font.clone(),
                font_size: 24.,
                color: Color::WHITE,
            },
        ),
        DebugLine {},
    ));
    commands.spawn(
        TextBundle::from_section(
            "\"space\" - pause\n\"=\"/\"-\" - change game speed\n\"esc\" - hide help",
            TextStyle {
                font: font.clone(),
                font_size: 16.,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            // position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(25.0),
            ..default()
        }),
    );
}

// ,
// position: UiRect {
//     bottom: Val::Px(5.0),
//     right: Val::Px(15.0),
//     ..default()
// },
// ..default()

fn update_ui(
    settings: Res<Settings>,
    time_state: Res<State<TimeState>>,
    mut ev_update_ui: EventReader<UpdateUiEvent>,
    mut q: Query<&mut Text, With<DebugLine>>,
) {
    for _ev in ev_update_ui.read() {
        println!("update ui");

        let mut text = q.single_mut();
        text.sections[0].value = format_ui_line(&settings, &time_state);
    }
}

fn format_ui_line(settings: &Res<Settings>, time_state: &Res<State<TimeState>>) -> String {
    match time_state.get() {
        TimeState::Running => format!("Speed: {}x", settings.time_scale),
        TimeState::Paused => "Paused".to_string(),
    }
}
