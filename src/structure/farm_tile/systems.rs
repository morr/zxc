use super::*;

pub fn progress_on_progress_event(
    mut query: Query<(&mut FarmTile, &Transform)>,
    mut commands: Commands,
    mut event_reader: EventReader<FarmTileProgressEvent>,
    assets: Res<FarmAssets>,
    mut state_change_event_writer: EventWriter<EntityStateChangeEvent<FarmTileState>>,
) {
    for event in event_reader.read() {
        let entity = event.0;
        let (mut farm_tile, transform) = query.get_mut(entity).unwrap();

        farm_tile.progress_state(
            entity,
            &mut commands,
            transform,
            &assets,
            &mut state_change_event_writer,
        );
    }
}

pub fn progress_timer(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut FarmTile, &Transform), With<farm_tile_state::Planted>>,
    mut commands: Commands,
    assets: Res<FarmAssets>,
    mut state_change_event_writer: EventWriter<EntityStateChangeEvent<FarmTileState>>,
) {
    for (entity, mut farm_tile, transform) in query.iter_mut() {
        let timer = match &mut farm_tile.state {
            FarmTileState::Planted(timer) => timer,
            _ => panic!("FarmTile must be in a timer-assigned state"),
        };
        timer.tick(time_scale.scale_to_duration(time.delta_seconds()));

        if timer.finished() {
            farm_tile.progress_state(
                entity,
                &mut commands,
                transform,
                &assets,
                &mut state_change_event_writer,
            );
        }
    }
}

pub fn progress_on_state_change(
    mut event_reader: EventReader<EntityStateChangeEvent<FarmTileState>>,
    mut work_queue: ResMut<TasksQueue>,
    query: Query<&Transform>,
) {
    for event in event_reader.read() {
        let entity = event.0;
        let state = &event.1;

        let maybe_task_kind = match state {
            FarmTileState::NotPlanted => Some(TaskKind::FarmTilePlant),
            FarmTileState::Grown => Some(TaskKind::FarmTileHarvest),
            _ => None,
        };

        if let Some(task_kind) = maybe_task_kind {
            let transform = query.get(entity).unwrap();
            let grid_tile = transform.world_pos_to_grid();

            work_queue.add_task(Task {
                entity,
                kind: task_kind,
                grid_tile,
            });
        }
    }
}
