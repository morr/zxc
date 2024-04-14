use super::*;

pub fn apply_movement(
    mut commands: Commands,
    mut query_movement: Query<(Entity, &mut Movement, &mut Transform), With<MovementMoving>>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut movement_state_event_writer: EventWriter<EntityStateChangeEvent<MovementState>>,
) {
    for (entity, mut movement, mut transform) in &mut query_movement {
        let distance_to_move = movement.speed * time.delta_seconds() * time_scale.0;
        move_to_target_location(&mut movement, &mut transform, distance_to_move);
        if movement.path.is_empty() {
            movement.to_idle(entity, &mut commands, &mut movement_state_event_writer);
        }
    }
}

fn move_to_target_location(
    movement: &mut Movement,
    transform: &mut Transform,
    distance_to_move: f32,
) {
    let current_point_world: Vec2 = transform.translation.truncate();

    if let Some(target_point_tile) = movement.path.front() {
        let target_point_world = target_point_tile.grid_tile_center_to_world();
        let direction = (target_point_world - current_point_world).normalize_or_zero();
        let distance_between_points = (target_point_world - current_point_world).length();

        if distance_to_move >= distance_between_points {
            transform.translation = target_point_world.extend(transform.translation.z);
            movement.path.pop_front();

            let remaining_distance = distance_to_move - distance_between_points;

            if remaining_distance > 0.0 {
                move_to_target_location(movement, transform, remaining_distance);
            }
        } else {
            transform.translation += (direction * distance_to_move).extend(0.0);
        }
    }
}
