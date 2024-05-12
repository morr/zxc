use super::*;

pub fn render_debug_info(
    mut commands: Commands,
    assets: Res<FontAssets>,
    tasks_queue: Res<TasksQueue>,
    async_queue_counter: Res<AsyncQueueCounter>,
) {
    commands
        .spawn(NodeBundle {
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
        })
        .with_children(|container_parent| {
            container_parent.spawn((
                TextBundle::from_section(
                    format_debug_line(&tasks_queue, &async_queue_counter),
                    TextStyle {
                        font: assets.fira.clone(),
                        font_size: 18.,
                        color: Color::WHITE,
                    },
                ),
                DebugStatusTextUIMarker::default(),
            ));

            container_parent.spawn((
                TextBundle::from_section(
                    "\"space\" - pause
\"=\"/\"-\" - change game speed
\"h\" - toggle help
\"g\" - toggle grid
\"n\" - toggle navmesh
\"m\" - toggle movepath",
                    TextStyle {
                        font: assets.fira.clone(),
                        font_size: 12.,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect {
                        top: Val::Px(8.0),
                        right: Val::Px(0.0),
                        bottom: Val::Px(0.0),
                        left: Val::Px(0.0),
                    },
                    ..default()
                }),
                DebugHelpBlockUIMarker::default(),
            ));
        });
}

pub fn update_debug_info(
    tasks_queue: Res<TasksQueue>,
    async_queue_counter: Res<AsyncQueueCounter>,
    mut query: Query<&mut Text, With<DebugStatusTextUIMarker>>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format_debug_line(&tasks_queue, &async_queue_counter);
}

fn format_debug_line(
    tasks_queue: &Res<TasksQueue>,
    async_queue_counter: &Res<AsyncQueueCounter>,
) -> String {
    format!(
        "TasksQueue: {} AsyncQueue: {}",
        tasks_queue.len(),
        async_queue_counter.get()
    )
}

#[allow(clippy::too_many_arguments)]
pub fn handle_debug_info_keys(
    // mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    // query: Query<Entity>,
    mut query: Query<(&mut Visibility, &mut Style), With<DebugHelpBlockUIMarker>>,
    debug_grid_state: Res<State<DebugGridState>>,
    mut next_debug_grid_state: ResMut<NextState<DebugGridState>>,
    debug_navmesh_state: Res<State<DebugNavmeshState>>,
    mut next_debug_navmesh_state: ResMut<NextState<DebugNavmeshState>>,
    mut state_change_event_writer: EventWriter<StateChangeEvent<DebugNavmeshState>>,
    debug_movepath_state: Res<State<DebugMovepathState>>,
    mut next_debug_movepath_state: ResMut<NextState<DebugMovepathState>>,
) {
    if keys.just_pressed(KeyCode::KeyH) {
        // commands.entity(query.single_mut()).iis
        let (mut visibility, mut style) = query.single_mut();

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
        state_change_event_writer.send(StateChangeEvent(new_state));
    }

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
