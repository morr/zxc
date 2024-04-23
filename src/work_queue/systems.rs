use crate::*;

pub fn assign_tasks_to_pawns(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Pawn), With<PawnIdle>>,
    mut work_queue: ResMut<WorkQueue>,
    mut event_writer: EventWriter<EntityStateChangeEvent<PawnStatus>>
) {
    for (entity, mut pawn) in query.iter_mut() {
        if pawn.task.is_none() {
            if let Some(task) = work_queue.get_task() {
                pawn.task = Some(task);
                pawn.change_status(entity, PawnStatus::WorkAssigned, &mut commands, &mut event_writer);
            }
        }
    }
}
