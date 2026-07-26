use crate::assets::FontAssets;
use crate::prelude::*;
use crate::story_time::current_day_hour;
use crate::story_time::current_hour_minute;
use crate::story_time::current_year;
use crate::story_time::current_year_day;
use crate::story_time::year_day_to_season_day_label;
use crate::ui::UiOpacity;
use crate::ui::ui_color;
#[derive(Component, Default)]
struct SimulationSpeedTextUIMarker {}

#[derive(Component, Default)]
struct SimulationDateTimeTextUIMarker {}

pub struct UiSimulationStatePlugin;

impl Plugin for UiSimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_simulation_speed_ui)
            .add_systems(
                Update,
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
) {
    commands.spawn((
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
        children![
            (
                Text(format_simulation_speed_text(&time)),
                TextFont {
                    font: font_assets.fira.clone().into(),
                    font_size: FontSize::Px(24.),
                    ..default()
                },
                TextColor(Color::WHITE),
                SimulationSpeedTextUIMarker::default(),
            ),
            (
                Text(format_date_time_text(&time)),
                TextFont {
                    font: font_assets.fira.clone().into(),
                    font_size: FontSize::Px(18.),
                    ..default()
                },
                TextColor(Color::WHITE),
                SimulationDateTimeTextUIMarker::default(),
            ),
        ],
    ));
}

fn update_simulation_speed_text(
    entity: Single<Entity, With<SimulationSpeedTextUIMarker>>,
    time: Res<Time<Virtual>>,
    mut writer: TextUiWriter,
) {
    *writer.text(*entity, 0) = format_simulation_speed_text(&time);
}

fn format_simulation_speed_text(time: &Res<Time<Virtual>>) -> String {
    if time.is_paused() {
        format!("Paused ({}x)", time.relative_speed())
    } else {
        format!("Speed: {}x", time.relative_speed())
    }
}

fn update_simulation_date_time_text(
    entity: Single<Entity, With<SimulationDateTimeTextUIMarker>>,
    time: Res<Time<Virtual>>,
    mut writer: TextUiWriter,
) {
    *writer.text(*entity, 0) = format_date_time_text(&time);
}

fn format_date_time_text(time: &Res<Time<Virtual>>) -> String {
    format!(
        "{}, {}y, {:02}:{:02}",
        year_day_to_season_day_label(current_year_day(time.elapsed_secs())),
        current_year(time.elapsed_secs()),
        current_day_hour(time.elapsed_secs()),
        current_hour_minute(time.elapsed_secs())
    )
}
