use super::*;
use bevy::ecs::system::EntityCommands;

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
    let mut root = commands.spawn(Node {
        position_type: PositionType::Absolute,
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(8.),
        top: UI_SCREEN_EDGE_PX_OFFSET,
        left: UI_SCREEN_EDGE_PX_OFFSET,
        ..default()
    });

    spawn_item::<PawnStockTextUIMarker>(
        &mut root,
        PawnStockTextUIMarker::default(),
        pawns_query.iter().count() as u32,
        font_assets.fira.clone(),
        icon_assets.pawns.clone(),
    );

    spawn_item::<FoodStockTextUIMarker>(
        &mut root,
        FoodStockTextUIMarker::default(),
        food.amount,
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
            .spawn((
                Node {
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
                BackgroundColor(ui_color(UiOpacity::Heavy)),
            ))
            .with_children(|parent| {
                parent.spawn((
                    ImageNode::new(image),
                    // material: materials.add(texture_handle.into()),
                    Node {
                        width: Val::Px(28.),
                        height: Val::Px(28.),
                        margin: UiRect {
                            top: Val::Px(0.),
                            right: Val::Px(8.),
                            bottom: Val::Px(0.),
                            left: Val::Px(0.),
                        },
                        ..default()
                        // size: Size::new(Val::Percent(100.0), Val::Percent(100.0)), // Image will fill the node
                    },
                ));

                parent.spawn((
                    Text(format_item_text(amount)),
                    TextFont {
                        font,
                        font_size: 20.,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    marker_component,
                ));
            });
    });
}

fn update_food_stock_text(
    query: Query<Entity, With<FoodStockTextUIMarker>>,
    mut writer: TextUiWriter,
    food_stock: Res<FoodStock>,
) {
    let entity = query.single();
    *writer.text(entity, 0) = format_item_text(food_stock.amount);
}

fn format_item_text(amount: u32) -> String {
    format!("{}", amount)
}

fn update_pawn_stock_text(
    text_query: Query<Entity, With<PawnStockTextUIMarker>>,
    mut writer: TextUiWriter,
    pawns_query: Query<&Pawn>,
) {
    let entity = text_query.single();
    *writer.text(entity, 0) = format_item_text(pawns_query.iter().count() as u32)
}
