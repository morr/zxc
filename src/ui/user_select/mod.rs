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
    mut user_select_event_reader: EventReader<UserSelectionChangeEvent>,
    user_selected_root_ui_query: Query<Entity, With<UserSelectedRootUIMarker>>,
    user_selection: Res<CurrentUserSelection>,
    pawn_query: Query<(&Pawn, &Movable, &Restable)>,
    farm_query: Query<(&Farm, &Workable)>,
    font_assets: Res<FontAssets>,
) {
    for _event in user_select_event_reader.read() {
        // println!("{:?}", event);
        let selected_root_ui_id = user_selected_root_ui_query.get_single().unwrap();
        let mut user_selected_root_ui_commands = commands.entity(selected_root_ui_id);
        user_selected_root_ui_commands.despawn_descendants();

        if let Some(UserSelectionData { id, kind }) = &user_selection.0 {
            match kind {
                UserSelectionKind::Pawn => {
                    if let Ok((pawn, movable, restable)) = pawn_query.get(*id) {
                        render_pawn_ui(
                            *id,
                            &mut user_selected_root_ui_commands,
                            pawn,
                            movable,
                            restable,
                            &font_assets,
                            UiOpacity::Heavy,
                        );
                    }
                }
                UserSelectionKind::Farm => {
                    if let Ok((farm, workable)) = farm_query.get(*id) {
                        render_farm_ui(
                            *id,
                            &mut user_selected_root_ui_commands,
                            farm,
                            workable,
                            &font_assets,
                            UiOpacity::Heavy,
                        );
                    }
                }
            }
        }
    }
}
