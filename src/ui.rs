use crate::{settings::Settings, TimeState};
use bevy::{ecs::query::QuerySingleError, prelude::*};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateUiEvent>()
            .add_systems(Startup, render_ui)
            .add_systems(FixedUpdate, update_ui)
            .add_systems(FixedUpdate, hide_help);
    }
}

#[derive(Event)]
pub struct UpdateUiEvent {}

#[derive(Component)]
pub struct DebugText {}

#[derive(Component)]
pub struct HelpText {}

fn render_ui(
    mut commands: Commands,
    settings: Res<Settings>,
    time_state: Res<State<TimeState>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        TextBundle::from_section(
            format_ui_line(&settings, &time_state),
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 24.,
                color: Color::WHITE,
            },
        ),
        DebugText {},
    ));
    spawn_help(&mut commands, &asset_server);
}

fn update_ui(
    settings: Res<Settings>,
    time_state: Res<State<TimeState>>,
    mut ev_update_ui: EventReader<UpdateUiEvent>,
    mut query: Query<&mut Text, With<DebugText>>,
) {
    for _ev in ev_update_ui.read() {
        println!("update ui");

        let mut text = query.single_mut();
        text.sections[0].value = format_ui_line(&settings, &time_state);
    }
}

fn format_ui_line(settings: &Res<Settings>, time_state: &Res<State<TimeState>>) -> String {
    match time_state.get() {
        TimeState::Running => format!("Speed: {}x", settings.time_scale),
        TimeState::Paused => "Paused".to_string(),
    }
}

fn hide_help(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    // query: Query<Entity>,
    query: Query<Entity, With<HelpText>>,
    asset_server: Res<AssetServer>,
) {
    if keys.just_pressed(KeyCode::KeyH) {
        match query.get_single() {
            Ok(entity) => {
                commands.entity(entity).despawn();
            }
            Err(QuerySingleError::NoEntities(_)) => {
                spawn_help(&mut commands, &asset_server);
            }
            Err(QuerySingleError::MultipleEntities(_)) => {
                panic!("Error: There is more than one help text!");
            }
        }
    }
}

fn spawn_help(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "\"space\" - pause
\"=\"/\"-\" - change game speed
\"h\" - toggle help
\"g\" - toggle grid",
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 16.,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            left: Val::Px(0.0),
            top: Val::Px(25.0),
            ..default()
        }),
        HelpText {},
    ));
}
