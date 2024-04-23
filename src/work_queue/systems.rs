use crate::*;

pub fn assign_tasks_to_pawns(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Pawn,
            &Transform,
            &mut Movement,
            Option<&mut PathfindingTask>,
        ),
        With<PawnIdle>,
    >,
    mut work_queue: ResMut<WorkQueue>,
    mut event_writer: EventWriter<EntityStateChangeEvent<PawnStatus>>,
    mut movement_state_event_writer: EventWriter<EntityStateChangeEvent<MovementState>>,
    arc_navmesh: Res<ArcNavmesh>,
    queue_counter: Res<AsyncQueueCounter>,
) {
    for (entity, mut pawn, transform, mut movement, mut maybe_pathfinding_task) in query.iter_mut()
    {
        if pawn.task.is_none() {
            if let Some(task) = work_queue.get_task() {
                let tile = task.tile;
                pawn.task = Some(task);
                pawn.change_status(
                    entity,
                    PawnStatus::WorkAssigned,
                    &mut commands,
                    &mut event_writer,
                );

                movement.to_pathfinding_async(
                    entity,
                    transform.translation.truncate().world_pos_to_grid(),
                    tile,
                    &arc_navmesh,
                    &queue_counter,
                    maybe_pathfinding_task.as_deref_mut(),
                    &mut commands,
                    &mut movement_state_event_writer,
                );
            }
        }
    }
}
