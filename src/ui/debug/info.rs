use self::ui::headline_text_bundle;

use super::*;

#[derive(Component, Default)]
pub struct DebugInfoUIMarker {}

#[derive(Component, Default)]
pub struct DebugUiContainerarker {}

#[derive(Component, Default)]
pub struct DebugUiHeadlineUIMarker {}

#[derive(Component, Default)]
pub struct DebugHelpBlockUIMarker {}

pub fn render_debug_ui_container(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: UI_WINDOWS_GAP,
            bottom: px(0.0),
            right: px(0.0),
            ..default()
        },
        DebugUiContainerarker::default(),
    ));
}

pub fn render_debug_ui_info(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    root_ui_query: Query<Entity, With<DebugUiContainerarker>>,
    async_queue_counter: Res<AsyncQueueCounter>,
) {
    let root_ui_id = root_ui_query.single().unwrap();
    let mut root_ui_commands = commands.entity(root_ui_id);

    root_ui_commands.with_children(|parent| {
        parent.spawn((
            (render_debug_ui_window_node_bundle::<DebugInfoUIMarker>()),
            children![
                (
                    headline_text_bundle(format_headline(&async_queue_counter), &font_assets),
                    DebugUiHeadlineUIMarker::default(),
                ),
                (
                    Text::new(
                        // \"r\" - rebuild map
                        "\"space\" - pause
\"=\"/\"-\" - change game speed
\"h\" - toggle help
\"g\" - toggle grid
\"p\" - cycle between noises
\"n\" - toggle navmesh
\"m\" - toggle movepath",
                    ),
                    TextFont {
                        font: font_assets.fira.clone(),
                        font_size: 12.,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect {
                            top: px(8.0),
                            right: px(0.0),
                            bottom: px(0.0),
                            left: px(0.0),
                        },
                        ..default()
                    },
                    DebugHelpBlockUIMarker::default(),
                )
            ],
        ));
    });
}

pub fn update_debug_ui_headline(
    async_queue_counter: Res<AsyncQueueCounter>,
    text_query: Query<Entity, With<DebugUiHeadlineUIMarker>>,
    mut writer: TextUiWriter,
) {
    let text_entity = text_query.single().unwrap();
    *writer.text(text_entity, 0) = format_headline(&async_queue_counter);
}

fn format_headline(async_queue_counter: &Res<AsyncQueueCounter>) -> String {
    format!("AsyncQueue: {}", async_queue_counter.get())
}

#[allow(clippy::too_many_arguments)]
pub fn handle_debug_info_keys(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    // query: Query<Entity>,
    mut query: Query<(&mut Visibility, &mut Node), With<DebugHelpBlockUIMarker>>,

    debug_grid_state: Res<State<DebugGridState>>,
    mut next_debug_grid_state: ResMut<NextState<DebugGridState>>,

    debug_navmesh_state: Res<State<DebugNavmeshState>>,
    mut next_debug_navmesh_state: ResMut<NextState<DebugNavmeshState>>,

    debug_noise_state: Res<State<DebugNoiseState>>,
    mut next_debug_noise_state: ResMut<NextState<DebugNoiseState>>,

    debug_movepath_state: Res<State<DebugMovepathState>>,
    mut next_debug_movepath_state: ResMut<NextState<DebugMovepathState>>,
    // mut rebuild_map_event_writer: MessageWriter<RebuildMapEvent>,
) {
    if keys.just_pressed(KeyCode::KeyH) {
        let (mut visibility, mut style) = query.single_mut().unwrap();

        match *visibility {
            Visibility::Hidden => {
                *visibility = Visibility::Visible;
                style.display = Display::Flex;
            }
            _ => {
                *visibility = Visibility::Hidden;
                style.display = Display::None;
            }
        }
    }

    if keys.just_pressed(KeyCode::KeyG) {
        match debug_grid_state.get() {
            DebugGridState::Visible => next_debug_grid_state.set(DebugGridState::Hidden),
            DebugGridState::Hidden => next_debug_grid_state.set(DebugGridState::Visible),
        };
    }

    if keys.just_pressed(KeyCode::KeyN) {
        let new_state = match debug_navmesh_state.get() {
            DebugNavmeshState::Visible => DebugNavmeshState::Hidden,
            DebugNavmeshState::Hidden => DebugNavmeshState::Visible,
        };
        next_debug_navmesh_state.set(new_state.clone());
        commands.trigger(log_event!(StateChangeEvent(new_state)));
    }

    if keys.just_pressed(KeyCode::KeyP) {
        let new_state = match debug_noise_state.get() {
            DebugNoiseState::Hidden => DebugNoiseState::HeightNoise,
            DebugNoiseState::HeightNoise => DebugNoiseState::HumidityNoise,
            DebugNoiseState::HumidityNoise => DebugNoiseState::PropsNoise,
            DebugNoiseState::PropsNoise => DebugNoiseState::Hidden,
        };
        next_debug_noise_state.set(new_state.clone());
        commands.trigger(log_event!(StateChangeEvent(new_state)));
    }

    // if keys.just_pressed(KeyCode::KeyR) {
    //     rebuild_map_event_writer.write(log_message!(RebuildMapEvent));
    // }

    if keys.just_pressed(KeyCode::KeyM) {
        match debug_movepath_state.get() {
            DebugMovepathState::Visible => {
                next_debug_movepath_state.set(DebugMovepathState::Hidden)
            }
            DebugMovepathState::Hidden => {
                next_debug_movepath_state.set(DebugMovepathState::Visible)
            }
        };
    }
}
