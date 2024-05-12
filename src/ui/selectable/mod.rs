use super::*;

expose_submodules!(pawn, farm);

#[derive(Component, Default)]
struct SelectableContainerUIMarker {}

pub struct UiSelectablePlugin;

impl Plugin for UiSelectablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(AppState::Loading), render_selectable_container)
            .add_plugins((UiPawnPlugin, UiFarmPlugin));
    }
}

fn render_selectable_container(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.),
                top: Val::Px(8.),
                left: Val::Px(100.),
                padding: UiRect {
                    top: Val::Px(10.0),
                    right: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    left: Val::Px(10.0),
                },
                ..default()
            },
            background_color: (*Color::hex("181a1c").unwrap().set_a(0.25)).into(),
            ..default()
        },
        SelectableContainerUIMarker::default(),
    ));
}
