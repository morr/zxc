use super::*;

pub fn progress_farm_tile_state(
    mut query: Query<(&Transform, &mut FarmTile)>,
    mut commands: Commands,
    mut event_reader: EventReader<FarmTileProgressEvent>,
    assets: Res<FarmAssets>,
    mut state_change_event_writer: EventWriter<EntityStateChangeEvent<FarmTileState>>,
) {
    for event in event_reader.read() {
        let entity = event.0;
        let (transform, mut farm_tile) = query.get_mut(entity).unwrap();

        farm_tile.progress_state(
            entity,
            &mut commands,
            transform,
            &assets,
            &mut state_change_event_writer,
        );
    }
}

pub fn progress_farm_tile_timer(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &Transform, &mut FarmTile), With<farm_tile_state::Planted>>,
    mut commands: Commands,
    assets: Res<FarmAssets>,
    mut state_change_event_writer: EventWriter<EntityStateChangeEvent<FarmTileState>>,
) {
    for (entity, transform, mut farm_tile) in query.iter_mut() {
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

pub fn track_farm_tiles_grown(
    mut event_reader: EventReader<EntityStateChangeEvent<FarmTileState>>,
    query: Query<(&mut FarmTile, &Transform)>,
) {
    // for event in event_reader.read() {
    //     let a = event.0;
    //     let b = event.1;
    // }
}
