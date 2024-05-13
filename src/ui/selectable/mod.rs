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
                top: Val::Px(8.),
                left: Val::Px(100.),
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

fn selectable_node_bunlde() -> NodeBundle {
    // (
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
            max_width: Val::Px(300.),
            ..default()
        },
        background_color: bg_color(UiOpacity::Heavy),
        ..default()
    }
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

fn property_text_bundle<T: Default>(text: String, font_assets: &Res<FontAssets>) -> (TextBundle, T) {
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
