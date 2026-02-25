use super::*;

#[derive(Component, Default)]
pub struct UserSelectedRootUIMarker {}

pub struct UiUserSelectedPlugin;

impl Plugin for UiUserSelectedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_selected_ui)
            .add_observer(on_user_selection);
    }
}

fn render_selected_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: px(25.),
            top: UI_SCREEN_EDGE_PX_OFFSET,
            left: px(100.),
            ..default()
        },
        UserSelectedRootUIMarker::default(),
    ));
}

fn on_user_selection(
    _event: On<UserSelectionEvent>,
    mut commands: Commands,
    user_selected_root_ui_query: Query<Entity, With<UserSelectedRootUIMarker>>,
    user_selection: Res<CurrentUserSelection>,
    pawn_query: Query<(&Pawn, &Movable, &Restable, &Feedable, &Commandable)>,
    farm_query: Query<(&Farm, &Workable)>,
    font_assets: Res<FontAssets>,
) {
    let selected_root_ui_id = user_selected_root_ui_query
        .single()
        .expect("UserSelectedRootUI query failed");
    let mut user_selected_root_ui_commands = commands.entity(selected_root_ui_id);
    user_selected_root_ui_commands.despawn_related::<Children>();

    if let Some(UserSelectionData { entity: id, kind }) = &user_selection.0 {
        match kind {
            UserSelectionKind::Pawn => {
                if let Ok((pawn, movable, restable, feedable, commandable)) = pawn_query.get(*id) {
                    render_pawn_ui(
                        *id,
                        &mut user_selected_root_ui_commands,
                        pawn,
                        movable,
                        restable,
                        feedable,
                        commandable,
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
