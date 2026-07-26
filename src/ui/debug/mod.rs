use crate::prelude::*;
mod grid;
mod info;
mod movepath;
mod navmesh;
mod noise;
mod tasks_queue;
mod toggles;
pub use self::{grid::*, info::*, movepath::*, navmesh::*, noise::*, tasks_queue::*, toggles::*};
use crate::ui::UiOpacity;
use crate::ui::ui_color;

pub struct UiDebugPlugin;

impl Plugin for UiDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DebugGridPlugin)
            .add_plugins(DebugNavmeshPlugin)
            .add_plugins(DebugNoisePlugin)
            .add_plugins(DebugMovepathPlugin)
            .add_systems(OnExit(AppState::Loading), render_debug_ui_container)
            .add_systems(
                OnExit(AppState::Loading),
                (render_tasks_ui, render_debug_ui_info, render_debug_toggles)
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
                (handle_debug_info_keys, update_toggle_buttons).run_if(in_state(AppState::Playing)),
            );
    }
}

pub fn debug_ui_window_node<T: Default>() -> (Node, BackgroundColor, T) {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            padding: UiRect {
                top: px(10.0),
                right: px(10.0),
                bottom: px(10.0),
                left: px(10.0),
            },
            ..default()
        },
        BackgroundColor(ui_color(UiOpacity::Light)),
        T::default(),
    )
}
