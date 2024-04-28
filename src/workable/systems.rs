use super::*;

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
