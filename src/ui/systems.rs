use super::*;

pub fn render_simulation_speed_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    elapsed_time: Res<ElapsedTime>,
    time_state: Res<State<SimulationState>>,
    time_scale: Res<TimeScale>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                right: Val::Px(0.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                min_width: Val::Px(270.),
                row_gap: Val::Px(3.),
                padding: UiRect {
                    top: Val::Px(25.),
                    right: Val::Px(25.),
                    bottom: Val::Px(10.),
                    left: Val::Px(10.),
                },
                ..default()
            },
            background_color: (*UI_COLOR.clone().set_a(0.65)).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format_simulation_speed_text(&time_state, &time_scale),
                    TextStyle {
                        font: font_assets.fira.clone(),
                        font_size: 24.,
                        color: Color::WHITE,
                    },
                ),
                SimulationSpeedTextUI {},
            ));
            parent.spawn((
                TextBundle::from_section(
                    format_date_time_text(&elapsed_time),
                    TextStyle {
                        font: font_assets.fira.clone(),
                        font_size: 18.,
                        color: Color::WHITE,
                    },
                ),
                SimulationDateTimeTextUI {},
            ));
            // parent.spawn((
            //     TextBundle::from_section(
            //         format_season_text(&elapsed_time),
            //         TextStyle {
            //             font: font_assets.fira.clone(),
            //             font_size: 18.,
            //             color: Color::WHITE,
            //         },
            //     ),
            //     SimulationSeasonText {},
            // ));
        });
}

pub fn update_simulation_speed_text(
    mut query: Query<&mut Text, With<SimulationSpeedTextUI>>,
    time_state: Res<State<SimulationState>>,
    time_scale: Res<TimeScale>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format_simulation_speed_text(&time_state, &time_scale);
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

pub fn update_simulation_date_time_text(
    mut query: Query<&mut Text, With<SimulationDateTimeTextUI>>,
    elapsed_time: Res<ElapsedTime>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format_date_time_text(&elapsed_time);
}

// pub fn update_simulation_season_text(
//     mut query: Query<&mut Text, With<SimulationSeasonText>>,
//     elapsed_time: Res<ElapsedTime>,
// ) {
//     let mut text = query.single_mut();
//     text.sections[0].value = format_season_text(&elapsed_time);
// }

fn format_date_time_text(elapsed_time: &Res<ElapsedTime>) -> String {
    format!(
        "{}, {}y, {:02}:{:02}",
        ElapsedTime::year_day_to_season_day_label(elapsed_time.year_day()),
        elapsed_time.year(),
        elapsed_time.day_hour(),
        elapsed_time.hour_minute()
    )
}

// fn format_season_text(elapsed_time: &Res<ElapsedTime>) -> String {
//     format!(
//         "{:?}, Year {}",
//         elapsed_time.year_season(),
//         elapsed_time.year()
//     )
// }

// pub fn render_simulation_season_ui(
//     mut commands: Commands,
//     font_assets: Res<FontAssets>,
//     elapsed_time: Res<ElapsedTime>,
// ) {
//     commands
//         .spawn(NodeBundle {
//             style: Style {
//                 width: Val::Percent(100.0),
//                 // height: Val::Px(100.0),
//                 justify_content: JustifyContent::Center,
//                 ..default()
//             },
//             ..default()
//         })
//         .with_children(|parent| {
//             parent
//                 .spawn(NodeBundle {
//                     style: Style {
//                         padding: UiRect {
//                             top: Val::Px(10.),
//                             right: Val::Px(10.),
//                            bottom: Val::Px(10.),
//                             left: Val::Px(15.),
//                         },
//                         min_width: Val::Px(325.),
//                         ..default()
//                     },
//                     background_color: (*UI_COLOR.clone().set_a(0.85)).into(),
//                     ..default()
//                 })
//                 .with_children(|parent| {
//                     parent.spawn((
//                         TextBundle::from_section(
//                             format_season_text(&elapsed_time),
//                             TextStyle {
//                                 font: font_assets.fira.clone(),
//                                 font_size: 24.,
//                                 color: Color::WHITE,
//                             },
//                         ),
//                         SimulationSeasonText {},
//                     ));
//                 });
//         });
// }
