use super::*;

pub fn move_moving_entities(
    mut commands: Commands,
    mut query_movement: Query<(Entity, &mut Movement, &mut Transform), With<MovementMoving>>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    arc_navmesh: Res<ArcNavmesh>,
    mut movement_state_event_writer: EventWriter<EntityStateChangeEvent<MovementState>>,
) {
    for (entity, mut movement, mut transform) in &mut query_movement {
        move_to_target_location(
            entity,
            &mut movement,
            &mut transform,
            time.delta_seconds() * time_scale.0,
            &arc_navmesh,
            &mut commands,
            &mut movement_state_event_writer,
        );
    }
}

fn move_to_target_location(
    entity: Entity,
    movement: &mut Movement,
    transform: &mut Transform,
    remaining_time: f32,
    arc_navmesh: &ArcNavmesh,
    commands: &mut Commands,
    event_writer: &mut EventWriter<EntityStateChangeEvent<MovementState>>,
) {
    if movement.path.is_empty() {
        movement.to_idle(entity, commands, event_writer);
        return;
    }

    let current_tile = transform.translation.truncate().world_pos_to_grid();
    let speed_modifier = arc_navmesh
        .read()
        .navtiles
        .get(current_tile.x, current_tile.y)
        .cost
        .map_or(DEFAULT_COST as f32, |cost| cost as f32 / COST_MULTIPLIER);

    let actual_speed = movement.speed * speed_modifier;
    let distance_to_move = actual_speed * remaining_time;

    let target_point_tile = movement.path.front().unwrap();
    let target_point_world = target_point_tile.grid_tile_center_to_world();
    let direction = (target_point_world - transform.translation.truncate()).normalize_or_zero();
    let distance_between_points = (target_point_world - transform.translation.truncate()).length();

    if distance_to_move >= distance_between_points {
        transform.translation = target_point_world.extend(transform.translation.z);
        movement.path.pop_front();
        let remaining_distance = distance_to_move - distance_between_points;
        let remaining_time = remaining_distance / actual_speed;

        if remaining_time > 0.0 && !movement.path.is_empty() {
            move_to_target_location(
                entity,
                movement,
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
}
