use crate::*;

expose_submodules!(
    bed,
    carryable,
    farm,
    pawn,
    storage,
    tile,
    hovered,
    items_stock,
    simulation_state,
    user_select,
    debug
);

pub static UI_COLOR: Lazy<Color> = Lazy::new(|| Color::from(Srgba::hex("181a1c").unwrap()));
pub static UI_SCREEN_EDGE_PX_OFFSET: Val = Val::Px(8.);
pub static UI_WINDOWS_GAP: Val = Val::Px(25.);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiSimulationStatePlugin,
            UiItemsStockPlugin,
            UiPawnPlugin,
            UiFarmPlugin,
            UiBedPlugin,
            UiCarryablePlugin,
            UiStoragePlugin,
            UiHoveredPlugin,
            UiUserSelectedPlugin,
            UiDebugPlugin,
        ));
    }
}

pub trait TargetEntityUiMarker {
    fn new(target_id: Entity) -> Self;
}

pub enum UiOpacity {
    Light,
    Medium,
    Heavy,
}

pub fn ui_color(opacity: UiOpacity) -> Color {
    let mut color = *UI_COLOR;
    color.set_alpha(match opacity {
        UiOpacity::Light => 0.25,
        UiOpacity::Medium => 0.65,
        UiOpacity::Heavy => 0.85,
    });
    color
}

// pub fn bg_color(opacity: UiOpacity) -> BackgroundColor {
//     ui_color(opacity).into()
// }

fn render_entity_node_bunlde<T: TargetEntityUiMarker>(
    target_id: Entity,
    opacity: UiOpacity,
) -> (Node, BackgroundColor, T) {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.),
            padding: UiRect {
                top: Val::Px(10.),
                right: Val::Px(10.),
                bottom: Val::Px(10.),
                left: Val::Px(10.),
            },
            // max_width: Val::Percent(25.0),
            width: Val::Px(300.),
            ..default()
        },
        BackgroundColor(ui_color(opacity)),
        T::new(target_id),
    )
}

fn render_entity_component_node_bunlde<T: Default>() -> (Node, T) {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        T::default(),
    )
}

fn headline_text_bundle(
    text: String,
    font_assets: &Res<FontAssets>,
) -> (Text, TextFont, TextColor) {
    (
        Text(text),
        TextFont {
            font: font_assets.fira.clone(),
            font_size: 18.,
            ..default()
        },
        TextColor(Color::WHITE),
    )
}

fn property_text_bundle<T: Default>(
    text: String,
    font_assets: &Res<FontAssets>,
) -> (Text, TextFont, TextColor, T) {
    (
        Text(text),
        TextFont {
            font: font_assets.fira.clone(),
            font_size: 16.,
            ..default()
        },
        TextColor(Color::WHITE),
        T::default(),
    )
}
