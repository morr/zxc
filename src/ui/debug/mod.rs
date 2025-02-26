use crate::*;

expose_submodules!(grid, movepath, navmesh, info, tasks_queue);

pub struct UiDebugPlugin;

impl Plugin for UiDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DebugGridPlugin)
            .add_plugins(DebugNavmeshPlugin)
            .add_plugins(DebugMovepathPlugin)
            .add_systems(OnExit(AppState::Loading), render_debug_ui_container)
            .add_systems(
                OnExit(AppState::Loading),
                (render_tasks_ui, render_debug_ui_info)
                    .chain()
                    .after(render_debug_ui_container),
            )
            .add_systems(
                FixedUpdate,
                (update_debug_tasks_queue, update_debug_ui_headline)
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                Update,
                handle_debug_info_keys.run_if(in_state(AppState::Playing)),
            );
    }
}

pub fn render_debug_ui_window_node_bundle<T: Default>() -> (Node, BackgroundColor, T) {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect {
                top: Val::Px(10.0),
                right: Val::Px(10.0),
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
            },
            ..default()
        },
        BackgroundColor(ui_color(UiOpacity::Light)),
        T::default(),
    )
}
