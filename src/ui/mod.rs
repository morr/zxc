use crate::story_time::{ElapsedTime, TimeScale, TimeState};
use bevy::{ecs::query::QuerySingleError, prelude::*};

mod debug_grid;
use self::debug_grid::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DebugGridPlugin)
            // .add_event::<UpdateUiEvent>()
            .add_systems(Startup, render_ui)
            .add_systems(FixedUpdate, update_ui)
            .add_systems(FixedUpdate, handle_ui_keys);
    }
}

// #[derive(Event)]
// pub struct UpdateUiEvent {}

#[derive(Component)]
pub struct TimeText {}

#[derive(Component)]
pub struct HelpText {}

fn render_ui(
    mut commands: Commands,
    elapsed_time: Res<ElapsedTime>,
    time_state: Res<State<TimeState>>,
    time_scale: Res<TimeScale>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        TextBundle::from_section(
            format_ui_line(&elapsed_time, &time_state, &time_scale),
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 24.,
                color: Color::WHITE,
            },
        ),
        TimeText {},
    ));
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
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(25.0),
            ..default()
        }),
        HelpText {},
    ));
}

fn update_ui(
    elapsed_time: Res<ElapsedTime>,
    time_state: Res<State<TimeState>>,
    time_scale: Res<TimeScale>,
    // mut ev_update_ui: EventReader<UpdateUiEvent>,
    mut query: Query<&mut Text, With<TimeText>>,
) {
    // for _ev in ev_update_ui.read() {
    //     println!("update ui");

    let mut text = query.single_mut();
    text.sections[0].value = format_ui_line(&elapsed_time, &time_state, &time_scale);
    // }
}

fn format_ui_line(
    elapsed_time: &Res<ElapsedTime>,
    time_state: &Res<State<TimeState>>,
    time_scale: &Res<TimeScale>,
) -> String {
    let speed_part = match time_state.get() {
        TimeState::Running => format!("Speed: {}x", time_scale.0),
        TimeState::Paused => "Paused".to_string(),
    };

    format!("Seconds: {} {}", elapsed_time.0.floor(), speed_part)
}

fn handle_ui_keys(
    // mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    // query: Query<Entity>,
    mut query: Query<&mut Visibility, With<HelpText>>,
    debug_grid_state: Res<State<DebugGridState>>,
    mut next_debug_grid_state: ResMut<NextState<DebugGridState>>,
) {
    if keys.just_pressed(KeyCode::KeyH) {
        // commands.entity(query.single_mut()).iis
        let mut visibility = query.single_mut();

        match *visibility {
            Visibility::Hidden => {
                *visibility = Visibility::Visible;
            }
            _ => {
                *visibility = Visibility::Hidden;
            }
        }
        // match query.get_single() {
        //     Ok(entity) => {
        //         commands.entity(entity).despawn();
        //     }
        //     Err(QuerySingleError::NoEntities(_)) => {
        //         spawn_help(&mut commands, &asset_server);
        //     }
        //     Err(QuerySingleError::MultipleEntities(_)) => {
        //         panic!("Error: There is more than one help text!");
        //     }
        // }
    }

    if keys.just_pressed(KeyCode::KeyG) {
        match debug_grid_state.get() {
            DebugGridState::Visible => next_debug_grid_state.set(DebugGridState::Hidden),
            DebugGridState::Hidden => next_debug_grid_state.set(DebugGridState::Visible),
        };
    }
}
