use super::*;
use bevy::ecs::system::EntityCommands;

#[derive(Component)]
struct PawnStockTextUIMarker {}

#[derive(Component)]
struct FoodStockTextUIMarker {}

pub struct UiItemsStockPlugin;

impl Plugin for UiItemsStockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_items_stock_ui)
            .add_systems(
                FixedUpdate,
                (update_food_stock_text, update_pawn_stock_text)
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}

fn render_items_stock_ui(
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

    spawn_item::<PawnStockTextUIMarker>(
        &mut root,
        PawnStockTextUIMarker {},
        pawns_query.iter().count() as u32,
        font_assets.fira.clone(),
        icon_assets.pawns.clone(),
    );

    spawn_item::<FoodStockTextUIMarker>(
        &mut root,
        FoodStockTextUIMarker {},
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

fn update_food_stock_text(
    mut query: Query<&mut Text, With<FoodStockTextUIMarker>>,
    food: Res<FoodStock>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format_item_text(food.0);
}

fn format_item_text(amount: u32) -> String {
    format!("{}", amount)
}

fn update_pawn_stock_text(
    mut text_query: Query<&mut Text, With<PawnStockTextUIMarker>>,
    pawns_query: Query<&Pawn>,
) {
    let mut text = text_query.single_mut();
    text.sections[0].value = format_item_text(pawns_query.iter().count() as u32);
}
