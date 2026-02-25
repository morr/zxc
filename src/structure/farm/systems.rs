use std::time::Duration;

use super::*;

pub fn on_farm_progress(
    event: On<FarmProgressEvent>,
    time: Res<Time<Virtual>>,
    mut query: Query<(&mut Farm, &mut Workable, &Transform)>,
    mut commands: Commands,
    assets: Res<FarmAssets>,
) {
    let FarmProgressEvent { entity } = *event;
    // println!("{:?}", FarmProgressEvent(entity));
    let (mut farm, mut workable, transform) = query
        .get_mut(entity)
        .expect("on_farm_progress: Farm entity query failed");

    farm.progress_state(
        entity,
        &mut workable,
        &mut commands,
        transform.world_pos_to_grid(),
        total_days(time.elapsed_secs()),
        &assets,
    );
}

pub fn on_farm_tended(
    event: On<FarmTendedEvent>,
    time: Res<Time<Virtual>>,
    mut query: Query<&mut Farm>,
    // component tags seems to be working unreliable
    // mut query: Query<&mut Farm, With<farm_state::Planted>>,
) {
    let FarmTendedEvent { entity } = *event;
    // println!("{:?}", FarmTendedEvent(*entity));
    let Ok(mut farm) = query.get_mut(entity) else {
        return;
    };
    ensure_state!(fn: FarmState::Planted(_), farm.state);

    farm.tendings_done += 1;
    if let FarmState::Planted(planted_state) = &mut farm.state {
        planted_state.tending_rest_timer.reset();
        planted_state.tending_rest_started_day = total_days(time.elapsed_secs());
    }
}

pub fn progress_planted_and_tending_rest_timers(
    time: Res<Time<Virtual>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Farm), With<farm_state::Planted>>,
    mut tasks_scheduler: MessageWriter<ScheduleTaskMessage>,
) {
    for (workable_entity, mut farm) in query.iter_mut() {
        let planted_state = match &mut farm.state {
            FarmState::Planted(state) => state,
            _ => panic!("Farm must be in a timer-assigned state"),
        };

        let delta = Duration::from_secs_f32(time.delta_secs());
        planted_state.growth_timer.tick(delta);

        if planted_state.growth_timer.is_finished() {
            commands.trigger(log_event!(FarmProgressEvent {
                entity: workable_entity
            }));
        }

        if !planted_state.tending_rest_timer.is_finished() {
            planted_state.tending_rest_timer.tick(delta);

            if planted_state.tending_rest_timer.is_finished() {
                // println!(
                //     "tending_timer finished. tending_rest_started_day:{} game_day:{}",
                //     planted_state.tending_rest_started_day,
                //     elapsed_time.game_day()
                // );

                if planted_state.tending_rest_started_day != total_days(time.elapsed_secs()) {
                    tasks_scheduler.write(ScheduleTaskMessage::push_back(Task(TaskKind::Work {
                        workable_entity,
                        work_kind: WorkKind::FarmTending,
                    })));
                } else {
                    planted_state.is_tending_pending_for_next_day = true;
                }
            }
        }
    }
}

pub fn progress_harvested_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Farm), With<farm_state::Harvested>>,
) {
    for (entity, mut farm) in query.iter_mut() {
        let state = match &mut farm.state {
            FarmState::Harvested(state) => state,
            _ => panic!("Farm must be in a timer-assigned state"),
        };

        let delta = Duration::from_secs_f32(time.delta_secs());
        state.rest_timer.tick(delta);

        if state.rest_timer.is_finished() {
            commands.trigger(log_event!(FarmProgressEvent { entity }));
        }
    }
}

pub fn on_farm_state_change(
    event: On<EntityStateChangeEvent<FarmState>>,
    mut commands: Commands,
    query: Query<(&Farm, &Transform)>,
    mut tasks_scheduler: MessageWriter<ScheduleTaskMessage>,
) {
    let EntityStateChangeEvent(workable_entity, ref state) = *event;
    let maybe_task_kind = match state {
        FarmState::NotPlanted => Some(WorkKind::FarmPlanting),
        FarmState::Grown => Some(WorkKind::FarmHarvest),
        _ => None,
    };

    if (maybe_task_kind.is_some() || matches!(state, FarmState::Harvested(_)))
        && let Ok((farm, transform)) = query.get(workable_entity)
    {
        let grid_tile = transform.world_pos_to_grid();

        if let Some(work_kind) = maybe_task_kind {
            tasks_scheduler.write(ScheduleTaskMessage::push_back(Task(TaskKind::Work {
                workable_entity,
                work_kind,
            })));
        }

        if let FarmState::Harvested(_) = state {
            commands.trigger(log_event!(SpawnCarryableEvent {
                kind: CarryableKind::Food,
                amount: farm.yield_amount(),
                grid_tile,
            }));
        };
    }
}

pub fn on_new_day(
    _event: On<NewDayEvent>,
    mut query: Query<(Entity, &mut Farm), With<farm_state::Planted>>,
    mut tasks_scheduler: MessageWriter<ScheduleTaskMessage>,
) {
    for (workable_entity, mut farm) in query.iter_mut() {
        if let FarmState::Planted(planted_state) = &mut farm.state
            && planted_state.is_tending_pending_for_next_day
        {
            tasks_scheduler.write(ScheduleTaskMessage::push_back(Task(TaskKind::Work {
                workable_entity,
                work_kind: WorkKind::FarmTending,
            })));

            planted_state.is_tending_pending_for_next_day = false;
        };
    }
}
