use super::*;

#[derive(Component, Default)]
pub struct SelectableContainerUIMarker {}

pub struct UiSelectablePlugin;

impl Plugin for UiSelectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_selectable_container);
        // .add_plugins((UiPawnPlugin, UiFarmPlugin));
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
