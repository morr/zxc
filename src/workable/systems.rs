use super::*;

pub fn progress_work(
    query_pawns: Query<(Entity, &Pawn), With<PawnWorking>>,
    mut query_workable: Query<&mut Workable>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
) {
    let elapsed_time = time.delta_seconds() * time_scale.0;

    for (entity, pawn) in query_pawns.iter() {
        if let Some(task) = &pawn.task {
            if let Ok(mut workable) = query_workable.get_mut(task.entity) {
                workable.perform_work(elapsed_time);
            } else {
                error!("Cannot find Workable {:?} stored in task", task.entity);
            }
        } else {
            error!("PawnWorking {:?} has no task", entity);
        }
    }
}
