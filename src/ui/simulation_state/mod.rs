use super::*;

#[derive(Component, Default)]
struct SimulationSpeedTextUIMarker {}

// #[derive(Component, Default)]
// struct SimulationDateTimeTextUIMarker {}

pub struct UiSimulationStatePlugin;

impl Plugin for UiSimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_simulation_speed_ui)
            .add_systems(
                FixedUpdate,
                (
                    update_simulation_speed_text,
                    update_simulation_date_time_text,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

fn render_simulation_speed_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    time: Res<Time<Virtual>>,
    // elapsed_time: Res<ElapsedTime>,
    // time_state: Res<State<SimulationState>>,
    // time_scale: Res<TimeScale>,
) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(0.),
                right: px(0.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                min_width: px(270.),
                row_gap: px(3.),
                padding: UiRect {
                    top: px(25.),
                    right: px(25.),
                    bottom: px(10.),
                    left: px(10.),
                },
                ..default()
            },
            BackgroundColor(ui_color(UiOpacity::Medium)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text(format_simulation_speed_text(&time)),
                TextFont {
                    font: font_assets.fira.clone(),
                    font_size: 24.,
                    ..default()
                },
                TextColor(Color::WHITE),
                SimulationSpeedTextUIMarker::default(),
            ));
            // parent.spawn((
            //     Text(format_date_time_text(&elapsed_time)),
            //     TextFont {
            //         font: font_assets.fira.clone(),
            //         font_size: 18.,
            //         ..default()
            //     },
            //     TextColor(Color::WHITE),
            //     SimulationDateTimeTextUIMarker::default(),
            // ));
        });
}

fn update_simulation_speed_text(
    query: Query<Entity, With<SimulationSpeedTextUIMarker>>,
    time: Res<Time<Virtual>>,
    mut writer: TextUiWriter,
) {
    let entity = query.single().unwrap();
    *writer.text(entity, 0) = format_simulation_speed_text(&time);
}

fn format_simulation_speed_text(time: &Res<Time<Virtual>>) -> String {
    if time.is_paused() {
        format!("Paused ({}x)", time.relative_speed())
    } else {
        format!("Speed: {}x", time.relative_speed())
    }
}

fn update_simulation_date_time_text(//     query: Query<Entity, With<SimulationDateTimeTextUIMarker>>,
//     mut writer: TextUiWriter,
//     elapsed_time: Res<ElapsedTime>,
) {
    //     let entity = query.single().unwrap();
    //     *writer.text(entity, 0) = format_date_time_text(&elapsed_time);
}

// fn format_date_time_text(elapsed_time: &Res<Time>) -> String {
//     format!(
//         "{}, {}y, {:02}:{:02}",
//         ElapsedTime::year_day_to_season_day_label(elapsed_time.year_day()),
//         elapsed_time.year(),
//         elapsed_time.day_hour(),
//         elapsed_time.hour_minute()
//     )
// }
