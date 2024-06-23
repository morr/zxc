use super::*;

pub fn progress_on_farm_progress_event(
    elapsed_time: Res<ElapsedTime>,
    mut event_reader: EventReader<FarmProgressEvent>,
    mut query: Query<(&mut Farm, &mut Workable, &Transform)>,
    mut commands: Commands,
    assets: Res<FarmAssets>,
    mut state_change_event_writer: EventWriter<EntityStateChangeEvent<FarmState>>,
    mut commandable_interrupt_writer: EventWriter<ExternalCommandInterruptEvent>,
) {
    for FarmProgressEvent(entity) in event_reader.read() {
        // println!("{:?}", FarmProgressEvent(entity));
        let (mut farm, mut workable, transform) = query.get_mut(*entity).unwrap();

        farm.progress_state(
            *entity,
            &mut workable,
            &mut commands,
            transform.world_pos_to_grid(),
            elapsed_time.total_days(),
            &assets,
            &mut state_change_event_writer,
            &mut commandable_interrupt_writer,
        );
    }
}

pub fn progress_on_farm_tended_event(
    elapsed_time: Res<ElapsedTime>,
    mut event_reader: EventReader<FarmTendedEvent>,
    mut query: Query<&mut Farm>,
    // component tags seems to be working unreliable
    // mut query: Query<&mut Farm, With<farm_state::Planted>>,
) {
    for FarmTendedEvent(entity) in event_reader.read() {
        // println!("{:?}", FarmTendedEvent(*entity));
        let Ok(mut farm) = query.get_mut(*entity) else {
            continue;
        };
        ensure_state!(FarmState::Planted(_), farm.state);

        farm.tendings_done += 1;
        if let FarmState::Planted(planted_state) = &mut farm.state {
            planted_state.tending_rest_timer.reset();
            planted_state.tending_rest_started_day = elapsed_time.total_days();
        }
    }
}

pub fn progress_planted_and_tending_rest_timers(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    elapsed_time: Res<ElapsedTime>,
    mut query: Query<(Entity, &mut Farm), With<farm_state::Planted>>,
    mut farm_progress_event_writer: EventWriter<FarmProgressEvent>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
) {
    for (workable_entity, mut farm) in query.iter_mut() {
        let planted_state = match &mut farm.state {
            FarmState::Planted(state) => state,
            _ => panic!("Farm must be in a timer-assigned state"),
        };

        let delta = time_scale.scale_to_duration(time.delta_seconds());
        planted_state.growth_timer.tick(delta);

        if planted_state.growth_timer.finished() {
            farm_progress_event_writer.send(log_event!(FarmProgressEvent(workable_entity)));
        }

        if !planted_state.tending_rest_timer.finished() {
            planted_state.tending_rest_timer.tick(delta);

            if planted_state.tending_rest_timer.finished() {
                // println!(
                //     "tending_timer finished. tending_rest_started_day:{} game_day:{}",
                //     planted_state.tending_rest_started_day,
                //     elapsed_time.game_day()
                // );

                if planted_state.tending_rest_started_day != elapsed_time.total_days() {
                    tasks_scheduler.send(ScheduleTaskEvent::push_back(Task(TaskKind::Work {
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
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut Farm), With<farm_state::Harvested>>,
    mut farm_progress_event_writer: EventWriter<FarmProgressEvent>,
) {
    for (entity, mut farm) in query.iter_mut() {
        let state = match &mut farm.state {
            FarmState::Harvested(state) => state,
            _ => panic!("Farm must be in a timer-assigned state"),
        };

        let delta = time_scale.scale_to_duration(time.delta_seconds());
        state.rest_timer.tick(delta);

        if state.rest_timer.finished() {
            farm_progress_event_writer.send(log_event!(FarmProgressEvent(entity)));
        }
    }
}

pub fn progress_on_state_changed(
    mut event_reader: EventReader<EntityStateChangeEvent<FarmState>>,
    query: Query<(&Farm, &Transform)>,
    mut spawn_food_event_writer: EventWriter<SpawnCarryableEvent>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
) {
    for EntityStateChangeEvent(workable_entity, state) in event_reader.read() {
        // println!("{:?}", event);

        let maybe_task_kind = match state {
            FarmState::NotPlanted => Some(WorkKind::FarmPlanting),
            FarmState::Grown => Some(WorkKind::FarmHarvest),
            _ => None,
        };

        if maybe_task_kind.is_some() || matches!(state, FarmState::Harvested(_)) {
            if let Ok((farm, transform)) = query.get(*workable_entity) {
                let grid_tile = transform.world_pos_to_grid();

                if let Some(work_kind) = maybe_task_kind {
                    tasks_scheduler.send(ScheduleTaskEvent::push_back(Task(TaskKind::Work {
                        workable_entity: *workable_entity,
                        work_kind,
                    })));
                }

                if let FarmState::Harvested(_) = state {
                    spawn_food_event_writer.send(log_event!(SpawnCarryableEvent {
                        kind: CarryableKind::Food,
                        amount: farm.yield_amount(),
                        grid_tile,
                    }));
                };
            }
        }
    }
}

pub fn progress_on_new_day(
    mut event_reader: EventReader<NewDayEvent>,
    mut query: Query<(Entity, &mut Farm), With<farm_state::Planted>>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
) {
    for _event in event_reader.read() {
        for (workable_entity, mut farm) in query.iter_mut() {
            if let FarmState::Planted(planted_state) = &mut farm.state {
                if planted_state.is_tending_pending_for_next_day {
                    tasks_scheduler.send(ScheduleTaskEvent::push_back(Task(TaskKind::Work {
                        workable_entity,
                        work_kind: WorkKind::FarmTending,
                    })));

                    planted_state.is_tending_pending_for_next_day = false;
                }
            };
        }
    }
}
