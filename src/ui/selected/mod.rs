use self::structure::Farm;

use super::*;

#[derive(Component, Default)]
pub struct SelectedRootUIMarker {}

pub struct UiSelectedPlugin;

impl Plugin for UiSelectedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_selected_ui)
            .add_systems(
                Update,
                update_ui_on_click_event.run_if(in_state(AppState::Playing)),
            );
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
        SelectedRootUIMarker::default(),
    ));
}

fn update_ui_on_click_event(
    mut commands: Commands,
    mut click_event_reader: EventReader<ClickEvent>,
    selected_root_ui_query: Query<Entity, With<SelectedRootUIMarker>>,
    pawn_query: Query<(Entity, &Pawn, &Movable), With<UserSelected>>,
    farm_query: Query<(Entity, &Farm, &Workable), With<UserSelected>>,
    font_assets: Res<FontAssets>,
) {
    for _event in click_event_reader.read() {
        let selected_root_ui_id = selected_root_ui_query.get_single().unwrap();
        let mut selected_root_ui_commands = commands.entity(selected_root_ui_id);
        selected_root_ui_commands.despawn_descendants();

        for (pawn_id, pawn, movable) in pawn_query.iter() {
            render_pawn_ui(
                pawn_id,
                &mut selected_root_ui_commands,
                pawn,
                movable,
                &font_assets,
            );
        }

        for (farm_id, farm, workable) in farm_query.iter() {
            render_farm_ui(
                farm_id,
                &mut selected_root_ui_commands,
                farm,
                workable,
                &font_assets,
            );
        }
    }
}
