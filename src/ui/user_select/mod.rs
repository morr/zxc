use self::structure::Farm;

use super::*;

#[derive(Component, Default)]
pub struct UserSelectedRootUIMarker {}

pub struct UiUserSelectedPlugin;

impl Plugin for UiUserSelectedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_selected_ui)
            .add_systems(
                Update,
                update_ui_on_user_select_event.run_if(in_state(AppState::Playing)),
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
        UserSelectedRootUIMarker::default(),
    ));
}

fn update_ui_on_user_select_event(
    mut commands: Commands,
    mut user_select_event_reader: EventReader<UserSelectEvent>,
    user_selected_root_ui_query: Query<Entity, With<UserSelectedRootUIMarker>>,
    pawn_query: Query<(Entity, &Pawn, &Movable), With<UserSelect>>,
    farm_query: Query<(Entity, &Farm, &Workable), With<UserSelect>>,
    font_assets: Res<FontAssets>,
) {
    for event in user_select_event_reader.read() {
        println!("{:?}", event);
        let selected_root_ui_id = user_selected_root_ui_query.get_single().unwrap();
        let mut user_selected_root_ui_commands = commands.entity(selected_root_ui_id);
        user_selected_root_ui_commands.despawn_descendants();

        for (pawn_id, pawn, movable) in pawn_query.iter() {
            render_pawn_ui(
                pawn_id,
                &mut user_selected_root_ui_commands,
                pawn,
                movable,
                &font_assets,
                UiOpacity::Heavy,
            );
        }

        for (farm_id, farm, workable) in farm_query.iter() {
            render_farm_ui(
                farm_id,
                &mut user_selected_root_ui_commands,
                farm,
                workable,
                &font_assets,
                UiOpacity::Heavy,
            );
        }
    }
}
