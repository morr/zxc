use super::*;

pub fn progress_work(
    mut commands: Commands,
    mut workable_query: Query<
        // (Entity, &mut Workable, &Commandable),
        (Entity, &mut Workable),
        With<workable_state::WorkableStateBeingWorkedTag>,
    >,
    time: Res<Time>,
) {
    let elapsed_time = time.delta_secs();

    for (workable_entity, mut workable) in workable_query.iter_mut() {
        ensure_state!(loop: WorkableState::BeingWorked(_), workable.state);

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
                panic!(
                    "Workable completed but previous state was not BeingWorked: {:?}",
                    prev_state
                )
            };

            commands.trigger(log_event!(WorkCompleteEvent {
                commandable_entity,
                workable_entity,
                work_kind
            }));
        }
    }
}

pub fn on_work_complete(event: On<WorkCompleteEvent>, mut commands: Commands) {
    let WorkCompleteEvent {
        workable_entity,
        ref work_kind,
        ..
    } = *event;

    match work_kind {
        // event.workable_entity the same is task.entity
        WorkKind::FarmPlanting | WorkKind::FarmHarvest => {
            commands.trigger(log_event!(FarmProgressEvent {
                entity: workable_entity
            }));
        }
        WorkKind::FarmTending => {
            commands.trigger(log_event!(FarmTendedEvent {
                entity: workable_entity
            }));
        }
    }
}
