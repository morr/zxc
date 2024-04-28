use super::*;

pub fn progress_work(
    query_pawns: Query<&Pawn, With<PawnWorking>>,
    mut query_workable: Query<&mut Workable>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
) {
    let elapsed_time = time.delta_seconds() * time_scale.0;

    for pawn in query_pawns.iter() {
        let task = pawn.get_task();

        let mut workable = query_workable.get_mut(task.entity).unwrap();

        workable.perform_work(elapsed_time);
    }
}
