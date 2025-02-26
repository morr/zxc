use super::*;

#[derive(Component, Default)]
struct SimulationSpeedTextUIMarker {}

#[derive(Component, Default)]
struct SimulationDateTimeTextUIMarker {}

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
    elapsed_time: Res<ElapsedTime>,
    time_state: Res<State<SimulationState>>,
    time_scale: Res<TimeScale>,
) {
    commands
        .spawn((
            Node {
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
            BackgroundColor(ui_color(UiOpacity::Medium)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text(format_simulation_speed_text(&time_state, &time_scale)),
                TextFont {
                    font: font_assets.fira.clone(),
                    font_size: 24.,
                    ..default()
                },
                TextColor(Color::WHITE),
                SimulationSpeedTextUIMarker::default(),
            ));
            parent.spawn((
                Text(format_date_time_text(&elapsed_time)),
                TextFont {
                    font: font_assets.fira.clone(),
                    font_size: 18.,
                    ..default()
                },
                TextColor(Color::WHITE),
                SimulationDateTimeTextUIMarker::default(),
            ));
        });
}

fn update_simulation_speed_text(
    query: Query<Entity, With<SimulationSpeedTextUIMarker>>,
    mut writer: TextUiWriter,
    time_state: Res<State<SimulationState>>,
    time_scale: Res<TimeScale>,
) {
    let entity = query.single();
    *writer.text(entity, 0) = format_simulation_speed_text(&time_state, &time_scale);
}

fn format_simulation_speed_text(
    time_state: &Res<State<SimulationState>>,
    time_scale: &Res<TimeScale>,
) -> String {
    match time_state.get() {
        SimulationState::Running => format!("Speed: {}x", time_scale.0),
        SimulationState::Paused => {
            if time_scale.0 > 1.0 {
                format!("Paused ({}x)", time_scale.0)
            } else {
                "Paused (1x)".to_string()
            }
        }
    }
}

fn update_simulation_date_time_text(
    query: Query<Entity, With<SimulationDateTimeTextUIMarker>>,
    mut writer: TextUiWriter,
    elapsed_time: Res<ElapsedTime>,
) {
    let entity = query.single();
    *writer.text(entity, 0) = format_date_time_text(&elapsed_time);
}

fn format_date_time_text(elapsed_time: &Res<ElapsedTime>) -> String {
    format!(
        "{}, {}y, {:02}:{:02}",
        ElapsedTime::year_day_to_season_day_label(elapsed_time.year_day()),
        elapsed_time.year(),
        elapsed_time.day_hour(),
        elapsed_time.hour_minute()
    )
}
