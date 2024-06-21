use super::*;

pub fn progress_work(
    mut commands: Commands,
    mut workable_query: Query<
        // (Entity, &mut Workable, &Commandable),
        (Entity, &mut Workable),
        With<workable_state::WorkableStateBeingWorkedTag>,
    >,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut event_writer: EventWriter<WorkCompleteEvent>,
) {
    let elapsed_time = time_scale.scale_to_seconds(time.delta_seconds());

    for (workable_entity, mut workable) in workable_query.iter_mut() {
        // --- problem here
        // --- workable was replaced by sync_workable
        ensure_state!(WorkableState::BeingWorked(_), workable.state);

        workable.perform_work(elapsed_time);

        if workable.is_work_complete() {
            workable.reset_amount_done();
            let prev_state =
                workable.change_state(WorkableState::Idle, workable_entity, &mut commands);

            let WorkableState::BeingWorked(WorkOnCommand {
                commandable_entity,
                task,
            }) = prev_state
            else {
                panic!()
            };

            event_writer.send(log_event!(WorkCompleteEvent {
                commandable_entity,
                task,
            }));
        }
    }
}

pub fn complete_work(
    // mut commands: Commands,
    mut event_reader: EventReader<WorkCompleteEvent>,
    // mut query: Query<&mut Pawn>,
    // mut state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
    mut farm_progress_event_writer: EventWriter<FarmProgressEvent>,
    mut farm_tending_event_writer: EventWriter<FarmTendedEvent>,
) {
    for WorkCompleteEvent {
        commandable_entity: _,
        task,
    } in event_reader.read()
    {
        // println!("{:?}", WorkCompleteEvent { commandable_entity, task });
        //
        // let mut pawn = query.get_mut(event.pawn_entity).unwrap();
        // let task = pawn.get_task();
        //
        //
        match task.work_kind {
            // event.workable_entity the same is task.entity
            WorkKind::FarmPlanting | WorkKind::FarmHarvest => {
                farm_progress_event_writer
                    .send(log_event!(FarmProgressEvent(task.workable_entity)));
            }
            WorkKind::FarmTending => {
                farm_tending_event_writer.send(log_event!(FarmTendedEvent(task.workable_entity)));
            }
        }
        //
        // pawn.change_state(
        //     PawnState::Idle,
        //     event.pawn_entity,
        //     &mut commands,
        //     // &mut state_change_event_writer,
        // );
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
//             event_writer.send(log_event!(WorkCompleteEvent {
//                 pawn_entity,
//                 workable_entity,
//             }));
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
//                 farm_progress_event_writer.send(log_event!(FarmProgressEvent(event.workable_entity)));
//             }
//             TaskKind::FarmTending => {
//                 farm_tending_event_writer.send(log_event!(FarmTendedEvent(event.workable_entity)));
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
