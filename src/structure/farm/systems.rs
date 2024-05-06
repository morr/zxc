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
            elapsed_time.game_day(),
            &assets,
            &mut state_change_event_writer,
        );
    }
}

pub fn progress_on_farm_tended_event(
    mut event_reader: EventReader<FarmTendedEvent>,
    mut query: Query<&mut Farm, With<farm_state::Planted>>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);

        if let Ok(mut farm) = query.get_mut(event.0) {
            farm.tendings_done += 1;
            if let FarmState::Planted(planted_state) = &mut farm.state {
                planted_state.tending_rest_timer.reset();
            }
        }
    }
}

pub fn progress_planted_and_tending_rest_timers(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    // elapsed_time: Res<ElapsedTime>,
    mut query: Query<(Entity, &mut Farm, &Transform), With<farm_state::Planted>>,
    mut farm_progress_event_writer: EventWriter<FarmProgressEvent>,
    mut work_queue: ResMut<TasksQueue>,
) {
    for (entity, mut farm, transform) in query.iter_mut() {
        let state = match &mut farm.state {
            FarmState::Planted(state) => state,
            _ => panic!("Farm must be in a timer-assigned state"),
        };

        let delta = time_scale.scale_to_duration(time.delta_seconds());
        state.growth_timer.tick(delta);

        if !state.tending_rest_timer.finished() {
            state.tending_rest_timer.tick(delta);

            if state.tending_rest_timer.finished() {
                // if state.tending_rest_started_day != elapsed_time.game_day() {
                work_queue.add_task(Task {
                    entity,
                    kind: TaskKind::FarmTending,
                    grid_tile: transform.world_pos_to_grid(),
                });
                // }
            }
        }

        if state.growth_timer.finished() {
            farm_progress_event_writer.send(FarmProgressEvent(entity));
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
    mut work_queue: ResMut<TasksQueue>,
    query: Query<(&Farm, &Transform)>,
    mut spawn_food_event_writer: EventWriter<SpawnItemEvent>,
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
                    work_queue.add_task(Task {
                        entity,
                        kind: task_kind,
                        grid_tile,
                    });
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

pub fn progress_on_new_day(// mut event_reader: EventReader<NewDayEvent>,
    // query: Query<&Farm, With<farm_state::Planted>>,
) {
    // for event in event_reader.read() {
    //     println!("{:?}", event);
    //
    //     for farm in query.iter() {}
    // }
}
