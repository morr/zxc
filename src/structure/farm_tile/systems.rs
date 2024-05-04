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
        let state = match &mut farm_tile.state {
            FarmTileState::Planted(state) => state,
            _ => panic!("FarmTile must be in a timer-assigned state"),
        };
        state.growth_timer.tick(time_scale.scale_to_duration(time.delta_seconds()));

        if state.growth_timer.finished() {
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
    mut spawn_food_event_writer: EventWriter<SpawnItemEvent>,
) {
    for event in event_reader.read() {
        let entity = event.0;
        let state = &event.1;

        let task_kind = match state {
            FarmTileState::NotPlanted => TaskKind::FarmTilePlant,
            FarmTileState::Planted(_) => TaskKind::FarmTileTending,
            FarmTileState::Grown => TaskKind::FarmTileHarvest,
            FarmTileState::Harvested => TaskKind::FarmTileCleanup,
        };

        work_queue.add_task(Task {
            entity,
            kind: task_kind,
            grid_tile: entity_grid_tile(entity, &query),
        });

        // generate food
        if let FarmTileState::Harvested = state {
            spawn_food_event_writer.send(SpawnItemEvent {
                item_type: ItemType::Food,
                amount: 10,
                grid_tile: entity_grid_tile(entity, &query)
            });
        };
    }
}

fn entity_grid_tile(entity: Entity, query: &Query<&Transform>) -> IVec2 {
    query.get(entity).unwrap().world_pos_to_grid()
}
