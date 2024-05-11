use bevy::ecs::system::EntityCommands;
pub use once_cell::sync::Lazy;

use super::*;

static UI_COLOR: Lazy<Color> = Lazy::new(|| Color::hex("181a1c").unwrap());

pub fn render_simulation_season_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    elapsed_time: Res<ElapsedTime>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                // height: Val::Px(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect {
                            top: Val::Px(10.),
                            right: Val::Px(10.),
                            bottom: Val::Px(10.),
                            left: Val::Px(15.),
                        },
                        min_width: Val::Px(325.),
                        ..default()
                    },
                    background_color: (*UI_COLOR.clone().set_a(0.85)).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            format_season_text(&elapsed_time),
                            TextStyle {
                                font: font_assets.fira.clone(),
                                font_size: 24.,
                                color: Color::WHITE,
                            },
                        ),
                        SimulationSeasonText {},
                    ));
                });
        });
}

pub fn update_simulation_season_text(
    mut query: Query<&mut Text, With<SimulationSeasonText>>,
    elapsed_time: Res<ElapsedTime>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format_season_text(&elapsed_time);
}

fn format_season_text(elapsed_time: &Res<ElapsedTime>) -> String {
    format!(
        "Day {} of {:?}, Year {}",
        elapsed_time.season_day(),
        elapsed_time.year_season(),
        elapsed_time.year()
    )
}

pub fn render_simulation_speed_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    elapsed_time: Res<ElapsedTime>,
    time_state: Res<State<SimulationState>>,
    time_scale: Res<TimeScale>,
) {
    let container = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            top: Val::Px(0.),
            right: Val::Px(0.),
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
    };

    let simulation_speed_line_node = NodeBundle::default();
    let simulation_speed_line = TextBundle::from_section(
        format_simulation_speed_text(&time_state, &time_scale),
        TextStyle {
            font: font_assets.fira.clone(),
            font_size: 24.,
            color: Color::WHITE,
        },
    );

    let date_time_line_node = NodeBundle::default();
    let date_time_line = TextBundle::from_section(
        format_date_time_text(&elapsed_time),
        TextStyle {
            font: font_assets.fira.clone(),
            font_size: 24.,
            color: Color::WHITE,
        },
    );

    commands.spawn(container).with_children(|parent| {
        parent
            .spawn(simulation_speed_line_node)
            .with_children(|parent| {
                parent.spawn((simulation_speed_line, SimulationSpeedText {}));
            });

        parent.spawn(date_time_line_node).with_children(|parent| {
            parent.spawn((date_time_line, SimulationDateTimeText {}));
        });
    });
}

pub fn render_items_stock_ui(
    mut commands: Commands,
    pawns_query: Query<&Pawn>,
    icon_assets: Res<IconAssets>,
    font_assets: Res<FontAssets>,
    food: Res<FoodStock>,
) {
    let mut root = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.),
            top: Val::Px(8.),
            left: Val::Px(8.),
            ..default()
        },
        ..default()
    });

    spawn_item::<PawnStockText>(
        &mut root,
        PawnStockText {},
        pawns_query.iter().count() as u32,
        font_assets.fira.clone(),
        icon_assets.pawns.clone(),
    );

    spawn_item::<FoodStockText>(
        &mut root,
        FoodStockText {},
        food.0,
        font_assets.fira.clone(),
        icon_assets.bread.clone(),
    );
}

fn spawn_item<T: Component>(
    root: &mut EntityCommands,
    marker_component: T,
    amount: u32,
    font: Handle<Font>,
    image: Handle<Image>,
) {
    root.with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    padding: UiRect {
                        top: Val::Px(3.),
                        right: Val::Px(10.),
                        bottom: Val::Px(3.),
                        left: Val::Px(10.),
                    },
                    ..default()
                },
                background_color: (*UI_COLOR.clone().set_a(0.85)).into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(28.),
                        height: Val::Px(28.),
                        margin: UiRect {
                            top: Val::Px(0.),
                            right: Val::Px(8.),
                            bottom: Val::Px(0.),
                            left: Val::Px(0.),
                        },
                        ..default() // size: Size::new(Val::Percent(100.0), Val::Percent(100.0)), // Image will fill the node
                    },
                    // material: materials.add(texture_handle.into()),
                    image: image.into(),
                    ..default()
                });

                parent.spawn((
                    TextBundle::from_section(
                        format_item_text(amount),
                        TextStyle {
                            font,
                            font_size: 20.,
                            color: Color::WHITE,
                        },
                    ),
                    marker_component,
                ));
            });
    });
}

pub fn update_simulation_speed_text(
    mut query: Query<&mut Text, With<SimulationSpeedText>>,
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
    mut query: Query<&mut Text, With<SimulationDateTimeText>>,
    elapsed_time: Res<ElapsedTime>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format_date_time_text(&elapsed_time);
}

fn format_date_time_text(elapsed_time: &Res<ElapsedTime>) -> String {
    format!(
        "Day {} {:02}:{:02}",
        elapsed_time.total_days(),
        elapsed_time.day_hour(),
        elapsed_time.hour_minute(),
    )
}

pub fn update_food_stock_text(
    mut query: Query<&mut Text, With<FoodStockText>>,
    food: Res<FoodStock>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format_item_text(food.0);
}

fn format_item_text(amount: u32) -> String {
    format!("{}", amount)
}

pub fn update_pawn_stock_text(
    mut text_query: Query<&mut Text, With<PawnStockText>>,
    pawns_query: Query<&Pawn>,
) {
    let mut text = text_query.single_mut();
    text.sections[0].value = format_item_text(pawns_query.iter().count() as u32);
}
