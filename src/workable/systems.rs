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
    mut event_writer: MessageWriter<WorkCompleteMessage>,
) {
    let elapsed_time = time_scale.scale_to_seconds(time.delta_secs());

    for (workable_entity, mut workable) in workable_query.iter_mut() {
        ensure_state!(WorkableState::BeingWorked(_), workable.state);

        workable.perform_work(elapsed_time);

        if workable.is_work_complete() {
            workable.reset_amount_done();
            let prev_state =
                workable.change_state(WorkableState::Idle, workable_entity, &mut commands);

            let WorkableState::BeingWorked(WorkOnCommand {
                commandable_entity,
                workable_entity,
                work_kind,
            }) = prev_state
            else {
                panic!()
            };

            event_writer.write(log_message!(WorkCompleteMessage {
                commandable_entity,
                workable_entity,
                work_kind
            }));
        }
    }
}

pub fn complete_work(
    mut event_reader: MessageReader<WorkCompleteMessage>,
    mut farm_progress_event_writer: MessageWriter<FarmProgressMessage>,
    mut farm_tending_event_writer: MessageWriter<FarmTendedMessage>,
) {
    for WorkCompleteMessage {
        workable_entity,
        work_kind,
        ..
    } in event_reader.read()
    {
        match work_kind {
            // event.workable_entity the same is task.entity
            WorkKind::FarmPlanting | WorkKind::FarmHarvest => {
                farm_progress_event_writer.write(log_message!(FarmProgressMessage(*workable_entity)));
            }
            WorkKind::FarmTending => {
                farm_tending_event_writer.write(log_message!(FarmTendedMessage(*workable_entity)));
            }
        }
    }
}

// pub fn start_pawn_working(
//     mut commands: Commands,
//     mut event_reader: MessageReader<WorkStartEvent>,
//     mut query: Query<&mut Pawn>,
//     // mut state_change_event_writer: MessageWriter<EntityStateChangeMessage<PawnState>>,
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
//     mut event_writer: MessageWriter<WorkCompleteEvent>,
// ) {
//     let elapsed_time = time_scale.scale_to_seconds(time.delta_secs());
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
//             event_writer.write(log_message!(WorkCompleteEvent {
//                 pawn_entity,
//                 workable_entity,
//             }));
//         }
//     }
// }

// pub fn complete_pawn_working(
//     mut commands: Commands,
//     mut event_reader: MessageReader<WorkCompleteEvent>,
//     mut query: Query<&mut Pawn>,
//     // mut state_change_event_writer: MessageWriter<EntityStateChangeMessage<PawnState>>,
//     mut farm_progress_event_writer: MessageWriter<FarmProgressEvent>,
//     mut farm_tending_event_writer: MessageWriter<FarmTendedEvent>,
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
//                 farm_progress_event_writer.write(log_message!(FarmProgressEvent(event.workable_entity)));
//             }
//             TaskKind::FarmTending => {
//                 farm_tending_event_writer.write(log_message!(FarmTendedEvent(event.workable_entity)));
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
