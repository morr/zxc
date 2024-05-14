use super::*;

expose_submodules!(pawn, farm);

#[derive(Component, Default)]
struct SelectableContainerUIMarker {}

pub struct UiSelectablePlugin;

impl Plugin for UiSelectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_selectable_container)
            .add_plugins((UiPawnPlugin, UiFarmPlugin));
    }
}

fn render_selectable_container(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(25.),
                top: UI_SCREEN_EDGE_PX_OFFSET,
                left: UI_SCREEN_EDGE_PLUS_ITEM_STOCKS_PX_OFFSET,
                ..default()
            },
            ..default()
        },
        SelectableContainerUIMarker::default(),
    ));
}

fn selectble_id(container_query: &Query<Entity, With<SelectableContainerUIMarker>>) -> Entity {
    container_query.get_single().unwrap()
}

fn selectable_subnode_bunlde() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }
}

fn headline_text_bundle(text: &str, font_assets: &Res<FontAssets>) -> TextBundle {
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
