use super::*;

expose_submodules!(pawn, farm);

#[derive(Component, Default)]
struct SelectableUIMarker {}

pub struct UiSelectablePlugin;

impl Plugin for UiSelectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPawnPlugin, UiFarmPlugin))
            .add_systems(OnExit(AppState::Loading), render_container);
    }
}

fn render_container(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                bottom: Val::Px(0.0),
                right: Val::Px(0.0),
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
        SelectableUIMarker::default(),
    ));
}
