use super::*;

pub fn progress_on_progress_event(
    mut event_reader: EventReader<FarmTileProgressEvent>,
    mut query: Query<(&mut FarmTile, &Transform)>,
    mut commands: Commands,
    assets: Res<FarmAssets>,
    mut state_change_event_writer: EventWriter<EntityStateChangeEvent<FarmTileState>>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);
        let entity = event.0;
        let (mut farm_tile, transform) = query.get_mut(entity).unwrap();

        farm_tile.progress_state(
            entity,
            &mut commands,
            transform.world_pos_to_grid(),
            &assets,
            &mut state_change_event_writer,
        );
    }
}

pub fn progress_on_tending_event(
    mut event_reader: EventReader<FarmTileTendedEvent>,
    mut query: Query<&mut FarmTile>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);
        if let Ok(mut farm_tile) = query.get_mut(event.0) {
            farm_tile.tendings_done += 1;
        }
    }
}

pub fn progress_planted_timer(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut FarmTile, &Transform), With<farm_tile_state::Planted>>,
    mut commands: Commands,
    assets: Res<FarmAssets>,
    mut state_change_event_writer: EventWriter<EntityStateChangeEvent<FarmTileState>>,
    mut work_queue: ResMut<TasksQueue>,
) {
    for (entity, mut farm_tile, transform) in query.iter_mut() {
        let state = match &mut farm_tile.state {
            FarmTileState::Planted(state) => state,
            _ => panic!("FarmTile must be in a timer-assigned state"),
        };
        let delta = time_scale.scale_to_duration(time.delta_seconds());
        state.growth_timer.tick(delta);

        if let Some(tending_rest_timer) = &mut state.tending_rest_timer {
            tending_rest_timer.tick(delta);

            if tending_rest_timer.finished() {
                work_queue.add_task(Task {
                    entity,
                    kind: TaskKind::FarmTileTending,
                    grid_tile: transform.world_pos_to_grid(),
                });
                state.tending_rest_timer = None;
            }
        }

        if state.growth_timer.finished() {
            farm_tile.progress_state(
                entity,
                &mut commands,
                transform.world_pos_to_grid(),
                &assets,
                &mut state_change_event_writer,
            );
        }
    }
}

pub fn progress_on_state_changed(
    mut event_reader: EventReader<EntityStateChangeEvent<FarmTileState>>,
    mut work_queue: ResMut<TasksQueue>,
    query: Query<(&FarmTile, &Transform)>,
    mut spawn_food_event_writer: EventWriter<SpawnItemEvent>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);

        let entity = event.0;
        let state = &event.1;

        let maybe_task_kind = match state {
            FarmTileState::NotPlanted => Some(TaskKind::FarmTilePlant),
            FarmTileState::Grown => Some(TaskKind::FarmTileHarvest),
            FarmTileState::Harvested => Some(TaskKind::FarmTileCleanup),
            _ => None,
        };

        if maybe_task_kind.is_some() || matches!(state, FarmTileState::Harvested) {
            if let Ok((farm_tile, transform)) = query.get(entity) {
                if let Some(task_kind) = maybe_task_kind {
                    work_queue.add_task(Task {
                        entity,
                        kind: task_kind,
                        grid_tile: transform.world_pos_to_grid()
                    });
                }

                if let FarmTileState::Harvested = state {
                    println!("tendings done: {}", farm_tile.tendings_done);
                    spawn_food_event_writer.send(SpawnItemEvent {
                        item_type: ItemType::Food,
                        amount: 10 * farm_tile.tendings_done,
                        grid_tile: transform.world_pos_to_grid()
                    });
                };
            }
        }
    }
}
