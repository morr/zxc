use self::structure::FarmTileProgressEvent;

use super::*;

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
    mut work_queue: ResMut<TasksQueue>,
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
    mut event_writer: EventWriter<WorkStartEvent>,
) {
    for (entity, transform, pawn) in query.iter() {
        let current_tile = transform.translation.truncate().world_pos_to_grid();
        let is_pawn_reached_workplace = current_tile == pawn.get_task().tile;

        if is_pawn_reached_workplace {
            event_writer.send(WorkStartEvent {
                pawn_entity: entity,
            });
        }
    }
}

pub fn start_pawn_working(
    mut commands: Commands,
    mut event_reader: EventReader<WorkStartEvent>,
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

pub fn progress_work(
    query_pawns: Query<(Entity, &Pawn), With<PawnWorking>>,
    mut query_workable: Query<(Entity, &mut Workable)>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut event_writer: EventWriter<WorkCompleteEvent>,
) {
    let elapsed_time = time.delta_seconds() * time_scale.0;

    for (pawn_entity, pawn) in query_pawns.iter() {
        let task = pawn.get_task();
        let (workable_entity, mut workable) = query_workable.get_mut(task.entity).unwrap();
        workable.perform_work(elapsed_time);

        if workable.is_work_complete() {
            event_writer.send(WorkCompleteEvent {
                pawn_entity,
                workable_entity,
            });
        }
    }
}

pub fn complete_pawn_working(
    mut commands: Commands,
    mut event_reader: EventReader<WorkCompleteEvent>,
    mut query: Query<&mut Pawn>,
    mut pawn_state_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
    mut farm_progress_event_writer: EventWriter<FarmTileProgressEvent>,
) {
    for event in event_reader.read() {
        let mut pawn = query.get_mut(event.pawn_entity).unwrap();
        let task = pawn.get_task();

        match task.kind {
            TaskKind::Farming => {
                // event.workable_entity the same is task.entity
                farm_progress_event_writer.send(FarmTileProgressEvent(event.workable_entity));
            }
        }

        pawn.change_state(
            event.pawn_entity,
            PawnState::Idle,
            &mut commands,
            &mut pawn_state_event_writer,
        );
    }
}
