use super::*;

pub fn render_tasks_ui(
    mut commands: Commands,
    assets: Res<FontAssets>,
    root_ui_query: Query<Entity, With<DebugUiContainerarker>>,
    tasks_queue: Res<TasksQueue>,
    async_queue_counter: Res<AsyncQueueCounter>,
) {
    let root_ui_id = root_ui_query.get_single().unwrap();
    let mut root_ui_commands = commands.entity(root_ui_id);
}
