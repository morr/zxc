use self::ui::headline_text_bundle;

use super::*;

#[derive(Component, Default)]
pub struct DebugTasksUIMarker {}

#[derive(Component, Default)]
pub struct DebugTasksQueueHeadlineUIMarker {}

#[derive(Component, Default)]
pub struct DebugTasksQueueDetailsUIMarker {}

pub fn render_tasks_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    root_ui_query: Query<Entity, With<DebugUiContainerarker>>,
    tasks_queue: Res<TasksQueue>,
) {
    let root_ui_id = root_ui_query.single().expect("DebugUiContainer query failed");
    let mut root_ui_commands = commands.entity(root_ui_id);

    root_ui_commands.with_children(|parent| {
        parent.spawn((
            render_debug_ui_window_node_bundle::<DebugTasksUIMarker>(),
            children![
                (
                    headline_text_bundle(format_headline(&tasks_queue), &font_assets),
                    DebugTasksQueueHeadlineUIMarker::default(),
                ),
                (
                    Text(format_details(&tasks_queue)),
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
                    DebugTasksQueueDetailsUIMarker::default(),
                ),
            ],
        ));
    });
}

pub fn update_debug_tasks_queue(
    ui_query: Query<Entity, With<DebugTasksUIMarker>>,
    texts_query: Query<
        (
            Entity,
            Option<&DebugTasksQueueHeadlineUIMarker>,
            Option<&DebugTasksQueueDetailsUIMarker>,
        ),
        Or<(
            With<DebugTasksQueueHeadlineUIMarker>,
            With<DebugTasksQueueDetailsUIMarker>,
        )>,
    >,
    tasks_queue: Res<TasksQueue>,
    children_query: Query<&Children>,
    mut writer: TextUiWriter,
) {
    let ui_id = ui_query.single().expect("DebugTasksUI query failed");

    if let Ok(children) = children_query.get(ui_id) {
        for child in children.iter() {
            update_text_markers_recursive(
                child,
                &tasks_queue,
                &texts_query,
                &children_query,
                &mut writer,
            );
        }
    }
}

fn update_text_markers_recursive(
    entity: Entity,
    tasks_queue: &Res<TasksQueue>,
    texts_query: &Query<
        (
            Entity,
            Option<&DebugTasksQueueHeadlineUIMarker>,
            Option<&DebugTasksQueueDetailsUIMarker>,
        ),
        Or<(
            With<DebugTasksQueueHeadlineUIMarker>,
            With<DebugTasksQueueDetailsUIMarker>,
        )>,
    >,
    children_query: &Query<&Children>,
    writer: &mut TextUiWriter,
) {
    if let Ok((text_entity, headline_marker, details_marker)) = texts_query.get(entity) {
        if headline_marker.is_some() {
            *writer.text(text_entity, 0) = format_headline(tasks_queue);
        }
        if details_marker.is_some() {
            *writer.text(text_entity, 0) = format_details(tasks_queue);
        }
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            update_text_markers_recursive(child, tasks_queue, texts_query, children_query, writer);
        }
    }
}

fn format_headline(tasks_queue: &Res<TasksQueue>) -> String {
    format!("TasksQueue: {}", tasks_queue.len())
}

fn format_details(tasks_queue: &Res<TasksQueue>) -> String {
    tasks_queue
        .tasks
        .iter()
        .map(|task| format!("{:?}", task))
        .collect::<Vec<String>>()
        .join("\n")
}
