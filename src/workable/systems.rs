use super::*;

pub fn assign_tasks_to_pawns(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Commandable), With<pawn_state::Idle>>,
    mut work_queue: ResMut<TasksQueue>,
) {
    for (entity, mut commandable) in query.iter_mut() {
        let Some(task) = work_queue.get_task() else {
            continue;
        };
        // println!("assign_tasks_to_pawns {:?}", task);

        commandable.schedule_execution(
            [
                CommandType::MoveTo(MoveToCommand(entity, task.grid_tile)),
                CommandType::WorkOn(WorkOnCommand(entity, task)),
            ],
            entity,
            &mut commands,
        );
    }
}

pub fn start_pawn_working(
    mut commands: Commands,
    mut event_reader: EventReader<WorkStartEvent>,
    mut query: Query<&mut Pawn>,
    // mut state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for event in event_reader.read() {
        let mut pawn = query.get_mut(event.pawn_entity).unwrap();
        let task = pawn.get_task().clone();

        pawn.change_state(
            PawnState::Working(task),
            event.pawn_entity,
            &mut commands,
            // &mut state_change_event_writer,
        );
    }
}

pub fn progress_work(
    query_pawns: Query<(Entity, &Pawn), With<pawn_state::Working>>,
    mut query_workable: Query<(Entity, &mut Workable)>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut event_writer: EventWriter<WorkCompleteEvent>,
) {
    let elapsed_time = time_scale.scale_to_seconds(time.delta_seconds());

    for (pawn_entity, pawn) in query_pawns.iter() {
        let task = pawn.get_task();
        let (workable_entity, mut workable) = query_workable.get_mut(task.entity).unwrap();
        workable.perform_work(elapsed_time);

        if workable.is_work_complete() {
            // println!("work_complete {:?}", task);
            workable.reset_work_amount_done();

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
    // mut state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
    mut farm_progress_event_writer: EventWriter<FarmProgressEvent>,
    mut farm_tending_event_writer: EventWriter<FarmTendedEvent>,
) {
    for event in event_reader.read() {
        let mut pawn = query.get_mut(event.pawn_entity).unwrap();
        let task = pawn.get_task();

        // println!("{:?} {:?}", event, task);

        match task.kind {
            // event.workable_entity the same is task.entity
            TaskKind::FarmPlant | TaskKind::FarmHarvest => {
                farm_progress_event_writer.send(FarmProgressEvent(event.workable_entity));
            }
            TaskKind::FarmTending => {
                farm_tending_event_writer.send(FarmTendedEvent(event.workable_entity));
            }
        }

        pawn.change_state(
            PawnState::Idle,
            event.pawn_entity,
            &mut commands,
            // &mut state_change_event_writer,
        );
    }
}
