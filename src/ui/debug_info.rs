use self::{
    debug_grid::DebugGridState, debug_movepath::DebugMovepathState,
    debug_navmesh::DebugNavmeshState,
};

use super::*;

pub fn render_debug_info(
    mut commands: Commands,
    assets: Res<FontAssets>,
    food: Res<Food>,
    tasks_queue: Res<TasksQueue>,
    async_queue_counter: Res<AsyncQueueCounter>,
) {
    commands.spawn((
        TextBundle::from_section(
            format_ui_line(
                food.0,
                &tasks_queue,
                &async_queue_counter,
            ),
            TextStyle {
                font: assets.fira.clone(),
                font_size: 24.,
                color: Color::WHITE,
            },
        ),
        DebugStatusText {},
    ));
    commands.spawn((
        TextBundle::from_section(
            "\"space\" - pause
\"=\"/\"-\" - change game speed
\"h\" - toggle help
\"g\" - toggle grid
\"n\" - toggle navmesh
\"m\" - toggle movepath",
            TextStyle {
                font: assets.fira.clone(),
                font_size: 16.,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(25.0),
            ..default()
        }),
        DebugHelpText {},
    ));
}

pub fn update_debug_info(
    food: Res<Food>,
    tasks_queue: Res<TasksQueue>,
    async_queue_counter: Res<AsyncQueueCounter>,
    mut query: Query<&mut Text, With<DebugStatusText>>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format_ui_line(
        food.0,
        &tasks_queue,
        &async_queue_counter,
    );
}

fn format_ui_line(
    food_amount: u32,
    tasks_queue: &Res<TasksQueue>,
    async_queue_counter: &Res<AsyncQueueCounter>,
) -> String {
    format!(
        "Food: {} TasksQueue: {} AsyncQueue: {}",
        food_amount,
        tasks_queue.len(),
        async_queue_counter.get()
    )
}

#[allow(clippy::too_many_arguments)]
pub fn handle_debug_info_keys(
    // mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    // query: Query<Entity>,
    mut query: Query<&mut Visibility, With<DebugHelpText>>,
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
        let mut visibility = query.single_mut();

        match *visibility {
            Visibility::Hidden => {
                *visibility = Visibility::Visible;
            }
            _ => {
                *visibility = Visibility::Hidden;
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
