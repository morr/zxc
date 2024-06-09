use super::*;

pub fn move_moving_entities(
    mut commands: Commands,
    mut query_movable: Query<(Entity, &mut Movable, &mut Transform), With<MovableStateMovinTag>>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    arc_navmesh: Res<ArcNavmesh>,
    // mut movable_state_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
    mut destination_reached_event_writer: EventWriter<MovableReachedDestinationEvent>,
    mut occupation_change_event_writer: EventWriter<OccupationChangeEvent>,
) {
    for (entity, mut movable, mut transform) in &mut query_movable {
        match movable.state {
            MovableState::Moving(_) => {
                let current_tile = transform.translation.truncate().world_pos_to_grid();
                let final_tile = move_to_target_location(
                    entity,
                    &mut movable,
                    &mut transform,
                    time_scale.scale_to_seconds(time.delta_seconds()),
                    &arc_navmesh,
                    &mut commands,
                    &mut destination_reached_event_writer, // &mut movable_state_event_writer,
                );

                if current_tile != final_tile {
                    let mut navmesh = arc_navmesh.write();

                    navmesh.remove_occupant::<Movable>(&entity, current_tile.x, current_tile.y);
                    navmesh.add_occupant::<Movable>(entity, final_tile.x, final_tile.y);

                    occupation_change_event_writer
                        .send(OccupationChangeEvent(vec![current_tile, final_tile]));
                }
            }
            _ => {
                // This can happen between switching states while executing commands
                continue;
            }
        }
    }
}

fn move_to_target_location(
    entity: Entity,
    movable: &mut Movable,
    transform: &mut Transform,
    remaining_time: f32,
    arc_navmesh: &ArcNavmesh,
    commands: &mut Commands,
    event_writer: &mut EventWriter<MovableReachedDestinationEvent>,
    // event_writer: &mut EventWriter<EntityStateChangeEvent<MovableState>>,
) -> IVec2 {
    if movable.path.is_empty() {
        let current_tile = transform.translation.truncate().world_pos_to_grid();
        let maybe_event_writer = match movable.state {
            MovableState::Moving(target_tile)
            // | MovableState::Pathfinding(target_tile)
            // | MovableState::PathfindingError(target_tile)
                if current_tile == target_tile =>
            {
                Some(event_writer)
            }
            _ => None,
        };
        // println!(
        //     "EventWriter<MovableReachedDestinationEvent> current_tile:{:?}, target_tile:{:?}",
        //     current_tile,
        //     match movable.state {
        //         MovableState::Moving(target_tile) => Some(target_tile),
        //         _ => None,
        //     }
        // );
        movable.to_idle(entity, commands, maybe_event_writer);
        return current_tile;
    }

    let current_point_world = transform.translation.truncate();
    let current_tile = current_point_world.world_pos_to_grid();
    let speed_modifier = arc_navmesh
        .read()
        .navtiles
        .get(current_tile.x, current_tile.y)
        .cost
        .map_or(1.0, |cost| COST_MULTIPLIER / cost as f32);

    let actual_speed = movable.speed * speed_modifier;
    let distance_to_move = actual_speed * remaining_time;

    let target_point_tile = movable.path.front().unwrap();
    let target_point_world = target_point_tile.grid_tile_center_to_world();
    let direction = (target_point_world - current_point_world).normalize_or_zero();
    let distance_between_points = (target_point_world - current_point_world).length();

    if distance_to_move >= distance_between_points {
        transform.translation = target_point_world.extend(transform.translation.z);
        movable.path.pop_front();

        let remaining_distance = distance_to_move - distance_between_points;
        let remaining_time = remaining_distance / actual_speed;

        if remaining_time > 0.0 {
            return move_to_target_location(
                entity,
                movable,
                transform,
                remaining_time,
                arc_navmesh,
                commands,
                event_writer,
            );
        }
    } else {
        transform.translation += (direction * distance_to_move).extend(0.0);
    }

    transform.translation.truncate().world_pos_to_grid()
}

pub fn stop_movable_on_death(
    mut commands: Commands,
    mut event_reader: EventReader<PawnDeathEvent>,
    mut query: Query<&mut Movable>,
) {
    for PawnDeathEvent(entity) in event_reader.read() {
        // println!("{:?}", event);
        let Ok(mut movable) = query.get_mut(*entity) else {
            continue;
        };

        movable.to_idle(*entity, &mut commands, None);
    }
}
