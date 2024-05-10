use bevy::ecs::system::EntityCommands;

use super::*;

pub fn render_simulation_ui(
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
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            // width: Val::Px(270.0),
            // height: Val::Px(100.0),
            padding: UiRect {
                // top: Val::Px(50.0),
                // right: Val::Px(50.0),
                top: Val::Px(25.0),
                right: Val::Px(25.0),
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
            },
            ..default()
        },
        background_color: (*Color::hex("181a1c").unwrap().set_a(0.65)).into(),
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
    texture_assets: Res<TextureAssets>,
    font_assets: Res<FontAssets>,
    food: Res<FoodStock>,
) {
    let mut root = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            top: Val::Px(5.),
            left: Val::Px(5.),
            ..default()
        },
        ..default()
    });

    spawn_item(
        &mut root,
        FoodStockText {},
        food.0,
        font_assets.fira.clone(),
        texture_assets.bread.clone(),
    );
}

#[derive(Component)]
pub struct FoodStockText {}

fn spawn_item(
    root: &mut EntityCommands,
    placeholder_struct: dyn Component,
    amount: u32,
    font: Handle<Font>,
    image: Handle<Image>,
) {
    root.with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    top: Val::Px(5.),
                    left: Val::Px(5.),
                    padding: UiRect {
                        top: Val::Px(3.),
                        right: Val::Px(10.),
                        bottom: Val::Px(3.),
                        left: Val::Px(10.),
                    },
                    ..default()
                },
                background_color: (*Color::hex("181a1c").unwrap().set_a(0.85)).into(),
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
                    placeholder_struct,
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
        elapsed_time.game_day(),
        elapsed_time.game_hours(),
        elapsed_time.game_minutes(),
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
