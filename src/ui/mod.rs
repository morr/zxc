use crate::*;

expose_submodules!(
    bed,
    farm,
    hovered,
    items_stock,
    pawn,
    simulation_state,
    tile,
    user_select,
    debug
);

pub static UI_COLOR: Lazy<Color> = Lazy::new(|| Color::hex("181a1c").unwrap());
pub static UI_SCREEN_EDGE_PX_OFFSET: Val = Val::Px(8.);
pub static UI_WINDOWS_GAP: Val = Val::Px(25.);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiSimulationStatePlugin,
            UiItemsStockPlugin,
            UiUserSelectedPlugin,
            UiPawnPlugin,
            UiFarmPlugin,
            UiHoveredPlugin,
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
    *UI_COLOR.clone().set_a(match opacity {
        UiOpacity::Light => 0.25,
        UiOpacity::Medium => 0.65,
        UiOpacity::Heavy => 0.85,
    })
}

pub fn bg_color(opacity: UiOpacity) -> BackgroundColor {
    ui_color(opacity).into()
}

fn render_entity_node_bunlde<T: TargetEntityUiMarker>(
    target_id: Entity,
    opacity: UiOpacity,
) -> (NodeBundle, T) {
    (
        NodeBundle {
            style: Style {
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
            background_color: bg_color(opacity),
            ..default()
        },
        T::new(target_id),
    )
}

fn render_entity_component_node_bunlde<T: Default>() -> (NodeBundle, T) {
    (
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        T::default(),
    )
}

fn headline_text_bundle(text: String, font_assets: &Res<FontAssets>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: font_assets.fira.clone(),
            font_size: 18.,
            color: Color::WHITE,
        },
    )
}

fn property_text_bundle<T: Default>(
    text: String,
    font_assets: &Res<FontAssets>,
) -> (TextBundle, T) {
    (
        TextBundle::from_section(
            text,
            TextStyle {
                font: font_assets.fira.clone(),
                font_size: 16.,
                color: Color::WHITE,
            },
        ),
        T::default(),
    )
}
