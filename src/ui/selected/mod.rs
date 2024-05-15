use super::*;

#[derive(Component, Default)]
pub struct SelectedUIRootMarker {}

pub struct UiSelectedPlugin;

impl Plugin for UiSelectedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_selected_ui);
        // .add_plugins((UiPawnPlugin, UiFarmPlugin));
    }
}

fn render_selected_ui(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(25.),
                top: UI_SCREEN_EDGE_PX_OFFSET,
                left: Val::Px(100.),
                ..default()
            },
            ..default()
        },
        SelectedUIRootMarker::default(),
    ));
}
