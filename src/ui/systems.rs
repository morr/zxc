use super::*;

pub fn render_ui(
    mut commands: Commands,
    assets: Res<FontAssets>,
    elapsed_time: Res<ElapsedTime>,
    time_state: Res<State<SimulationState>>,
    time_scale: Res<TimeScale>,
) {
    let container = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            // width: Val::Px(270.0),
            // height: Val::Px(100.0),
            padding: UiRect {
                top: Val::Px(50.0),
                right: Val::Px(50.0),
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
            },
            ..default()
        },
        background_color: (*Color::hex("181a1c").unwrap().set_a(0.5)).into(),
        ..default()
    };

    let simulation_speed_line_node = NodeBundle::default();
    let simulation_speed_line = TextBundle::from_section(
        format_simulation_speed_text(&time_state, &time_scale),
        TextStyle {
            font: assets.fira.clone(),
            font_size: 24.,
            color: Color::WHITE,
        },
    );

    let date_time_line_node = NodeBundle::default();
    let date_time_line = TextBundle::from_section(
        format_date_time_text(&elapsed_time),
        TextStyle {
            font: assets.fira.clone(),
            font_size: 24.,
            color: Color::WHITE,
        },
    );

    commands.spawn(container).with_children(|container_parent| {
        container_parent
            .spawn(simulation_speed_line_node)
            .with_children(|parent| {
                parent.spawn((simulation_speed_line, SimulationSpeedText {}));
            });

        container_parent
            .spawn(date_time_line_node)
            .with_children(|parent| {
                parent.spawn((date_time_line, SimulationDateTimeText {}));
            });
    });
}

pub fn update_simulation_speed_text(
    mut simulation_speed_query: Query<&mut Text, With<SimulationSpeedText>>,
    time_state: Res<State<SimulationState>>,
    time_scale: Res<TimeScale>,
) {
    let mut simulation_speed_text = simulation_speed_query.single_mut();
    simulation_speed_text.sections[0].value =
        format_simulation_speed_text(&time_state, &time_scale);

}

pub fn update_simulation_date_time_text(
    mut simulation_date_time_query: Query<&mut Text, With<SimulationDateTimeText>>,
    elapsed_time: Res<ElapsedTime>,
) {
    let mut simulation_date_time_text = simulation_date_time_query.single_mut();
    simulation_date_time_text.sections[0].value = format_date_time_text(&elapsed_time);
}

fn format_simulation_speed_text(
    time_state: &Res<State<SimulationState>>,
    time_scale: &Res<TimeScale>,
) -> String {
    match time_state.get() {
        SimulationState::Running => format!("Speed: {}x", time_scale.0),
        SimulationState::Paused => "Paused".to_string(),
    }
}

fn format_date_time_text(elapsed_time: &Res<ElapsedTime>) -> String {
    format!(
        "Day {} {:02}:{:02}",
        elapsed_time.game_day(),
        elapsed_time.game_hours(),
        elapsed_time.game_minutes(),
    )
}
