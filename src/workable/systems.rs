use super::*;

pub fn assign_tasks_to_pawns(
    mut commands: Commands,
    mut query: Query<(Entity, &Pawn, &mut Commandable), With<pawn_state::PawnStateIdleTag>>,
    mut work_queue: ResMut<TasksQueue>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
) {
    for (entity, pawn, mut commandable) in query.iter_mut() {
        ensure_state!(PawnState::Idle, pawn.state);

        let Some(task) = work_queue.get_task() else {
            continue;
        };
        // println!("assign_tasks_to_pawns {:?}", task);

        commandable.set_queue(
            [
                CommandType::MoveTo(MoveToCommand(entity, task.grid_tile)),
                CommandType::WorkOn(WorkOnCommand(entity, task)),
            ],
            entity,
            &mut commands,
            &mut tasks_scheduler,
        );
    }
}

pub fn progress_work(
    mut commands: Commands,
    mut workable_query: Query<
        (Entity, &mut Workable),
        With<workable_state::WorkableStateBeingWorkedTag>,
    >,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut event_writer: EventWriter<WorkCompleteEvent>,
) {
    let elapsed_time = time_scale.scale_to_seconds(time.delta_seconds());

    for (workable_entity, mut workable) in workable_query.iter_mut() {
        ensure_state!(WorkableState::BeingWorked(_), workable.state);

        workable.perform_work(elapsed_time);

        if workable.is_work_complete() {
            let WorkableState::BeingWorked(commandable_entity) = workable.state else {
                panic!()
            };

            workable.change_state(WorkableState::Idle, workable_entity, &mut commands);
            workable.reset_amount_done();

            event_writer.send(WorkCompleteEvent {
                workable_entity,
                commandable_entity,
            });
        }
    }
}

// pub fn start_pawn_working(
//     mut commands: Commands,
//     mut event_reader: EventReader<WorkStartEvent>,
//     mut query: Query<&mut Pawn>,
//     // mut state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
// ) {
//     for event in event_reader.read() {
//         let mut pawn = query.get_mut(event.pawn_entity).unwrap();
//         let task = pawn.get_task().clone();
//
//         pawn.change_state(
//             PawnState::Working(task),
//             event.pawn_entity,
//             &mut commands,
//             // &mut state_change_event_writer,
//         );
//     }
// }

// pub fn progress_work(
//     pawns_query: Query<(Entity, &Pawn), With<pawn_state::PawnStateWorkingTag>>,
//     mut workable_query: Query<(Entity, &mut Workable)>,
//     time: Res<Time>,
//     time_scale: Res<TimeScale>,
//     mut event_writer: EventWriter<WorkCompleteEvent>,
// ) {
//     let elapsed_time = time_scale.scale_to_seconds(time.delta_seconds());
//
//     for (pawn_entity, pawn) in pawns_query.iter() {
//         let task = pawn.get_task();
//         let (workable_entity, mut workable) = workable_query.get_mut(task.entity).unwrap();
//         workable.perform_work(elapsed_time);
//
//         if workable.is_work_complete() {
//             // println!("work_complete {:?}", task);
//             workable.reset_amount_done();
//
//             event_writer.send(WorkCompleteEvent {
//                 pawn_entity,
//                 workable_entity,
//             });
//         }
//     }
// }

// pub fn complete_pawn_working(
//     mut commands: Commands,
//     mut event_reader: EventReader<WorkCompleteEvent>,
//     mut query: Query<&mut Pawn>,
//     // mut state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
//     mut farm_progress_event_writer: EventWriter<FarmProgressEvent>,
//     mut farm_tending_event_writer: EventWriter<FarmTendedEvent>,
// ) {
//     for event in event_reader.read() {
//         let mut pawn = query.get_mut(event.pawn_entity).unwrap();
//         let task = pawn.get_task();
//
//         // println!("{:?} {:?}", event, task);
//
//         match task.kind {
//             // event.workable_entity the same is task.entity
//             TaskKind::FarmPlant | TaskKind::FarmHarvest => {
//                 farm_progress_event_writer.send(FarmProgressEvent(event.workable_entity));
//             }
//             TaskKind::FarmTending => {
//                 farm_tending_event_writer.send(FarmTendedEvent(event.workable_entity));
//             }
//         }
//
//         pawn.change_state(
//             PawnState::Idle,
//             event.pawn_entity,
//             &mut commands,
//             // &mut state_change_event_writer,
//         );
//     }
// }
