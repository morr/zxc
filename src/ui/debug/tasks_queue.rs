use self::ui::headline_text_bundle;

use super::*;

#[derive(Component, Default)]
pub struct DebugTasksQueueHeadlineUIMarker {}

pub fn render_tasks_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    root_ui_query: Query<Entity, With<DebugUiContainerarker>>,
    tasks_queue: Res<TasksQueue>,
) {
    let root_ui_id = root_ui_query.get_single().unwrap();
    let mut root_ui_commands = commands.entity(root_ui_id);

    root_ui_commands.with_children(|parent| {
        parent
            .spawn(render_debug_ui_window_node_bundle())
            .with_children(|container_parent| {
                container_parent.spawn((
                    headline_text_bundle(
                        format_headline(&tasks_queue),
                        &font_assets,
                    ),
                    DebugTasksQueueHeadlineUIMarker::default(),
                ));

            });
    });
}

pub fn update_debug_tasks_queue(
    tasks_queue: Res<TasksQueue>,
    mut query: Query<&mut Text, With<DebugTasksQueueHeadlineUIMarker>>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format_headline(&tasks_queue);
}

fn format_headline(tasks_queue: &Res<TasksQueue>) -> String {
    format!("TasksQueue: {}", tasks_queue.len())
}

