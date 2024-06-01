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
    let root_ui_id = root_ui_query.get_single().unwrap();
    let mut root_ui_commands = commands.entity(root_ui_id);

    root_ui_commands.with_children(|parent| {
        parent
            .spawn(render_debug_ui_window_node_bundle::<DebugTasksUIMarker>())
            .with_children(|container_parent| {
                container_parent.spawn((
                    headline_text_bundle(format_headline(&tasks_queue), &font_assets),
                    DebugTasksQueueHeadlineUIMarker::default(),
                ));

                container_parent.spawn((
                    TextBundle::from_section(
                        format_details(&tasks_queue),
                        TextStyle {
                            font: font_assets.fira.clone(),
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
                    DebugTasksQueueDetailsUIMarker::default(),
                ));
            });
    });
}

pub fn update_debug_tasks_queue(
    ui_query: Query<Entity, With<DebugTasksUIMarker>>,
    mut texts: Query<
        (
            &mut Text,
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
) {
    let ui_id = ui_query.get_single().unwrap();

    if let Ok(children) = children_query.get(ui_id) {
        for &child in children.iter() {
            update_text_markers_recursive(child, &tasks_queue, &mut texts, &children_query);
        }
    }
}

fn update_text_markers_recursive(
    entity: Entity,
    tasks_queue: &Res<TasksQueue>,
    texts: &mut Query<
        (
            &mut Text,
            Option<&DebugTasksQueueHeadlineUIMarker>,
            Option<&DebugTasksQueueDetailsUIMarker>,
        ),
        Or<(
            With<DebugTasksQueueHeadlineUIMarker>,
            With<DebugTasksQueueDetailsUIMarker>,
        )>,
    >,
    children_query: &Query<&Children>,
) {
    if let Ok((mut text, headline_marker, details_marker)) = texts.get_mut(entity) {
        if headline_marker.is_some() {
            text.sections[0].value = format_headline(tasks_queue);
        }
        if details_marker.is_some() {
            text.sections[0].value = format_details(tasks_queue);
        }
    }

    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            update_text_markers_recursive(child, tasks_queue, texts, children_query);
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
