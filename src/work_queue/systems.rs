use crate::*;

pub fn assign_tasks_to_pawns(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &mut Pawn,
            &Transform,
            &mut Movable,
            Option<&mut PathfindingTask>,
        ),
        With<PawnIdle>,
    >,
    mut work_queue: ResMut<WorkQueue>,
    mut pawn_state_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
    mut movable_state_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
    arc_navmesh: Res<ArcNavmesh>,
    queue_counter: Res<AsyncQueueCounter>,
) {
    for (entity, mut pawn, transform, mut movable, mut maybe_pathfinding_task) in query.iter_mut()
    {
        if pawn.task.is_none() {
            if let Some(task) = work_queue.get_task() {
                let tile = task.tile;

                pawn.task = Some(task);
                pawn.change_state(
                    entity,
                    PawnState::WorkAssigned,
                    &mut commands,
                    &mut pawn_state_event_writer,
                );

                movable.to_pathfinding_async(
                    entity,
                    transform.translation.truncate().world_pos_to_grid(),
                    tile,
                    &arc_navmesh,
                    &queue_counter,
                    maybe_pathfinding_task.as_deref_mut(),
                    &mut commands,
                    &mut movable_state_event_writer,
                );
            }
        }
    }
}

pub fn check_pawn_ready_for_working(
    query: Query<(Entity, &Transform, &Pawn), (With<PawnWorkAssigned>, Without<MovableMoving>)>,
    mut event_writer: EventWriter<PawnStartWorkingEvent>,
) {
    for (entity, transform, pawn) in query.iter() {
        let current_tile = transform.translation.truncate().world_pos_to_grid();
        let task_tile = pawn.task.as_ref().unwrap().tile;

        if current_tile == task_tile {
            event_writer.send(PawnStartWorkingEvent(entity));
        }
    }
}

pub fn start_pawn_working(
    mut commands: Commands,
    mut event_reader: EventReader<PawnStartWorkingEvent>,
    mut query: Query<&mut Pawn>,
    mut pawn_state_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for event in event_reader.read() {
        let mut pawn = query.get_mut(event.0).unwrap();

        pawn.change_state(
            event.0,
            PawnState::Working,
            &mut commands,
            &mut pawn_state_event_writer,
        );
    }
}
