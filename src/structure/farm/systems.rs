use super::*;

pub fn progress_on_farm_progress_event(
    elapsed_time: Res<ElapsedTime>,
    mut event_reader: EventReader<FarmProgressEvent>,
    mut query: Query<(&mut Farm, &Transform)>,
    mut commands: Commands,
    assets: Res<FarmAssets>,
    mut state_change_event_writer: EventWriter<EntityStateChangeEvent<FarmState>>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);
        let entity = event.0;
        let (mut farm, transform) = query.get_mut(entity).unwrap();

        farm.progress_state(
            entity,
            &mut commands,
            transform.world_pos_to_grid(),
            elapsed_time.total_days(),
            &assets,
            &mut state_change_event_writer,
        );
    }
}

pub fn progress_on_farm_tended_event(
    elapsed_time: Res<ElapsedTime>,
    mut event_reader: EventReader<FarmTendedEvent>,
    mut query: Query<&mut Farm, With<farm_state::Planted>>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);

        if let Ok(mut farm) = query.get_mut(event.0) {
            farm.tendings_done += 1;
            if let FarmState::Planted(planted_state) = &mut farm.state {
                planted_state.tending_rest_timer.reset();
                planted_state.tending_rest_started_day = elapsed_time.total_days();
            }
        }
    }
}

pub fn progress_planted_and_tending_rest_timers(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    elapsed_time: Res<ElapsedTime>,
    mut query: Query<(Entity, &mut Farm, &Transform), With<farm_state::Planted>>,
    mut farm_progress_event_writer: EventWriter<FarmProgressEvent>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
) {
    for (entity, mut farm, transform) in query.iter_mut() {
        let planted_state = match &mut farm.state {
            FarmState::Planted(state) => state,
            _ => panic!("Farm must be in a timer-assigned state"),
        };

        let delta = time_scale.scale_to_duration(time.delta_seconds());
        planted_state.growth_timer.tick(delta);

        if planted_state.growth_timer.finished() {
            farm_progress_event_writer.send(FarmProgressEvent(entity));
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
                    tasks_scheduler.send(ScheduleTaskEvent(
                        Task {
                            entity,
                            kind: TaskKind::FarmTending,
                            grid_tile: transform.world_pos_to_grid(),
                        },
                        QueuingType::PushBack,
                    ));
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
            farm_progress_event_writer.send(FarmProgressEvent(entity));
        }
    }
}

pub fn progress_on_state_changed(
    mut event_reader: EventReader<EntityStateChangeEvent<FarmState>>,
    query: Query<(&Farm, &Transform)>,
    mut spawn_food_event_writer: EventWriter<SpawnItemEvent>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);

        let entity = event.0;
        let state = &event.1;

        let maybe_task_kind = match state {
            FarmState::NotPlanted => Some(TaskKind::FarmPlant),
            FarmState::Grown => Some(TaskKind::FarmHarvest),
            _ => None,
        };

        if maybe_task_kind.is_some() || matches!(state, FarmState::Harvested(_)) {
            if let Ok((farm, transform)) = query.get(entity) {
                let grid_tile = transform.world_pos_to_grid();

                if let Some(task_kind) = maybe_task_kind {
                    tasks_scheduler.send(ScheduleTaskEvent(
                        Task {
                            entity,
                            kind: task_kind,
                            grid_tile,
                        },
                        QueuingType::PushBack,
                    ));
                }

                if let FarmState::Harvested(_) = state {
                    // println!("tendings done: {}", farm.tendings_done);

                    spawn_food_event_writer.send(SpawnItemEvent {
                        item_type: ItemType::Food,
                        amount: farm.yield_amount(),
                        grid_tile,
                    });
                };
            }
        }
    }
}

pub fn progress_on_new_day(
    mut event_reader: EventReader<NewDayEvent>,
    mut query: Query<(Entity, &mut Farm, &Transform), With<farm_state::Planted>>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
) {
    for _event in event_reader.read() {
        // println!("{:?}", event);

        for (entity, mut farm, transform) in query.iter_mut() {
            if let FarmState::Planted(planted_state) = &mut farm.state {
                if planted_state.is_tending_pending_for_next_day {
                    tasks_scheduler.send(ScheduleTaskEvent(
                        Task {
                            entity,
                            kind: TaskKind::FarmTending,
                            grid_tile: transform.world_pos_to_grid(),
                        },
                        QueuingType::PushBack,
                    ));

                    planted_state.is_tending_pending_for_next_day = false;
                }
            };
        }
    }
}
