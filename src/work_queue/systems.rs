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
    for (entity, mut pawn, transform, mut movable, mut maybe_pathfinding_task) in query.iter_mut() {
        if let Some(task) = work_queue.get_task() {
            let tile = task.tile;

            pawn.change_state(
                entity,
                PawnState::WorkAssigned(task),
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

pub fn check_pawn_ready_for_working(
    query: Query<(Entity, &Transform, &Pawn), (With<PawnWorkAssigned>, Without<MovableMoving>)>,
    mut event_writer: EventWriter<WorkStartingEvent>,
) {
    for (entity, transform, pawn) in query.iter() {
        let current_tile = transform.translation.truncate().world_pos_to_grid();
        let is_pawn_reached_workplace = current_tile == pawn.get_task().tile;

        if is_pawn_reached_workplace {
            event_writer.send(WorkStartingEvent {
                pawn_entity: entity,
            });
        }
    }
}

pub fn start_pawn_working(
    mut commands: Commands,
    mut event_reader: EventReader<WorkStartingEvent>,
    mut query: Query<&mut Pawn>,
    mut pawn_state_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for event in event_reader.read() {
        let mut pawn = query.get_mut(event.pawn_entity).unwrap();
        let task = pawn.get_task().clone();

        pawn.change_state(
            event.pawn_entity,
            PawnState::Working(task),
            &mut commands,
            &mut pawn_state_event_writer,
        );
    }
}
