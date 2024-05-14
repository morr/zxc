use super::*;

pub struct UiHoverMarkerPlugin;

impl Plugin for UiHoverMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_hovered_ui);
    }
}

fn render_hovered_ui(mut commands: Commands) {
    let root = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.),
            bottom: UI_SCREEN_EDGE_PX_OFFSET,
            left: UI_SCREEN_EDGE_PX_OFFSET,
            ..default()
        },
        ..default()
    });
}
