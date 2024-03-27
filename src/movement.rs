use std::collections::VecDeque;

use crate::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_moveable.run_if(in_state(TimeState::Running)));
    }
}

#[derive(Component)]
pub struct Moveable {
    pub path: VecDeque<IVec2>,
    pub speed: f32,
}

impl Moveable {
    pub fn new(speed: f32) -> Self {
        Self {
            path: VecDeque::new(),
            speed,
        }
    }
}

pub fn move_moveable(
    mut query_moveables: Query<(&mut Moveable, &mut Transform), With<Moveable>>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
) {
    for (mut moveable, mut transform) in &mut query_moveables {
        let distance_to_move = moveable.speed * time.delta_seconds() * time_scale.0;
        move_to_target_location(&mut moveable, &mut transform, distance_to_move);
    }
}

pub fn move_to_target_location(
    moveable: &mut Moveable,
    transform: &mut Transform,
    distance_to_move: f32,
) {
    let current_point_world: Vec2 = transform.translation.truncate();

    if let Some(target_point_tile) = moveable.path.front() {
        let target_point_world = target_point_tile.grid_tile_center_to_world();
        let direction = (target_point_world - current_point_world).normalize_or_zero();
        let distance_between_points = (target_point_world - current_point_world).length();

        if distance_to_move >= distance_between_points {
            transform.translation = target_point_world.extend(transform.translation.z);
            moveable.path.pop_front();

            let remaining_distance = distance_to_move - distance_between_points;

            if remaining_distance > 0.0 {
                move_to_target_location(moveable, transform, remaining_distance);
            }
        } else {
            transform.translation += (direction * distance_to_move).extend(0.0);
        }
    }
}
