use self::{debug_grid::DebugGridState, debug_movepath::DebugMovepathState, debug_navmesh::DebugNavmeshState};
use super::*;

pub fn render_ui(
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
        StatusText {},
    ));
    commands.spawn((
        TextBundle::from_section(
            "\"space\" - pause
\"=\"/\"-\" - change game speed
\"h\" - toggle help
\"g\" - toggle grid
\"n\" - toggle navmesh
\"m\" - toggle movepath",
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

pub fn update_ui(
    elapsed_time: Res<ElapsedTime>,
    time_state: Res<State<TimeState>>,
    time_scale: Res<TimeScale>,
    // mut ev_update_ui: EventReader<UpdateUiEvent>,
    mut query: Query<&mut Text, With<StatusText>>,
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

pub fn handle_ui_keys(
    // mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    // query: Query<Entity>,
    mut query: Query<&mut Visibility, With<HelpText>>,
    debug_grid_state: Res<State<DebugGridState>>,
    mut next_debug_grid_state: ResMut<NextState<DebugGridState>>,
    debug_navmesh_state: Res<State<DebugNavmeshState>>,
    mut next_debug_navmesh_state: ResMut<NextState<DebugNavmeshState>>,
    mut state_change_event_writer: EventWriter<StateChangeEvent<DebugNavmeshState>>,
    debug_movepath_state: Res<State<DebugMovepathState>>,
    mut next_debug_movepath_state: ResMut<NextState<DebugMovepathState>>,
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

    if keys.just_pressed(KeyCode::KeyN) {
        let new_state = match debug_navmesh_state.get() {
            DebugNavmeshState::Visible => DebugNavmeshState::Hidden,
            DebugNavmeshState::Hidden => DebugNavmeshState::Visible,
        };
        next_debug_navmesh_state.set(new_state.clone());
        state_change_event_writer.send(StateChangeEvent(new_state));
    }

    if keys.just_pressed(KeyCode::KeyM) {
        match debug_movepath_state.get() {
            DebugMovepathState::Visible => next_debug_movepath_state.set(DebugMovepathState::Hidden),
            DebugMovepathState::Hidden => next_debug_movepath_state.set(DebugMovepathState::Visible),
        };
    }
}
