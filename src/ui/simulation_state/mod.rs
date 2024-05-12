use super::*;

#[derive(Component)]
pub struct SimulationSpeedTextUI {}

#[derive(Component)]
pub struct SimulationDateTimeTextUI {}

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

fn format_date_time_text(elapsed_time: &Res<ElapsedTime>) -> String {
    format!(
        "{}, {}y, {:02}:{:02}",
        ElapsedTime::year_day_to_season_day_label(elapsed_time.year_day()),
        elapsed_time.year(),
        elapsed_time.day_hour(),
        elapsed_time.hour_minute()
    )
}
