use crate::assets::FontAssets;
use crate::assets::IconAssets;
use crate::carryable::FoodStock;
use crate::pawn::Pawn;
use crate::prelude::*;
use crate::ui::UI_SCREEN_EDGE_PX_OFFSET;
use crate::ui::UiOpacity;
use crate::ui::ui_color;
#[derive(Component, Default)]
struct PawnStockTextUIMarker {}

#[derive(Component, Default)]
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
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: px(8.),
            top: UI_SCREEN_EDGE_PX_OFFSET,
            left: UI_SCREEN_EDGE_PX_OFFSET,
            ..default()
        },
        children![
            item_node(
                PawnStockTextUIMarker::default(),
                pawns_query.iter().count() as u32,
                font_assets.fira.clone(),
                icon_assets.pawns.clone(),
            ),
            item_node(
                FoodStockTextUIMarker::default(),
                food.amount,
                font_assets.fira.clone(),
                icon_assets.bread.clone(),
            ),
        ],
    ));
}

fn item_node<T: Component>(
    marker_component: T,
    amount: u32,
    font: Handle<Font>,
    image: Handle<Image>,
) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            padding: UiRect {
                top: px(3.),
                right: px(10.),
                bottom: px(3.),
                left: px(10.),
            },
            ..default()
        },
        BackgroundColor(ui_color(UiOpacity::Heavy)),
        children![
            (
                ImageNode::new(image),
                Node {
                    width: px(28.),
                    height: px(28.),
                    margin: UiRect {
                        top: px(0.),
                        right: px(8.),
                        bottom: px(0.),
                        left: px(0.),
                    },
                    ..default()
                },
            ),
            (
                Text(format_item_text(amount)),
                TextFont {
                    font: font.into(),
                    font_size: FontSize::Px(20.),
                    ..default()
                },
                TextColor(Color::WHITE),
                marker_component,
            ),
        ],
    )
}

fn update_food_stock_text(
    entity: Single<Entity, With<FoodStockTextUIMarker>>,
    mut writer: TextUiWriter,
    food_stock: Res<FoodStock>,
) {
    *writer.text(*entity, 0) = format_item_text(food_stock.amount);
}

fn format_item_text(amount: u32) -> String {
    format!("{}", amount)
}

fn update_pawn_stock_text(
    entity: Single<Entity, With<PawnStockTextUIMarker>>,
    mut writer: TextUiWriter,
    pawns_query: Query<&Pawn>,
) {
    *writer.text(*entity, 0) = format_item_text(pawns_query.iter().count() as u32)
}
