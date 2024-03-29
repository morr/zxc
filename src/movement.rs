use std::collections::VecDeque;

use crate::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_movement.run_if(in_state(TimeState::Running)));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum MovementStatus {
    #[default]
    Idle,
    Moving,
    Pathfinding,
    PathfindingError,
}

#[derive(Debug, Component)]
pub struct Movement {
    pub path: VecDeque<IVec2>,
    pub speed: f32,
    pub status: MovementStatus,
}

#[derive(Component)]
pub struct MovementMoving;

impl Movement {
    pub fn new(speed: f32) -> Self {
        Self {
            path: VecDeque::new(),
            speed,
            status: MovementStatus::Idle,
        }
    }

    pub fn to_idle(&mut self, entity: Entity, commands: &mut Commands) {
        self.status = MovementStatus::Idle;
        commands.entity(entity).remove::<MovementMoving>();
    }

    pub fn to_moving(&mut self, path: VecDeque<IVec2>, entity: Entity, commands: &mut Commands) {
        self.path = path;
        self.status = MovementStatus::Moving;
        commands.entity(entity).insert(MovementMoving);
    }

    pub fn to_pathfinding(&mut self) {
        self.status = MovementStatus::Pathfinding;
    }

    pub fn to_pathfinding_error(&mut self) {
        self.status = MovementStatus::PathfindingError;
    }
}

// #[derive(Bundle)]
// pub struct MovementBundle {
//     pub movement: Movement,
//     // pub pathfind_status: PathfindStatus,
// }

pub fn apply_movement(
    mut commands: Commands,
    mut query_movement: Query<(Entity, &mut Movement, &mut Transform), With<MovementMoving>>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
) {
    for (entity, mut movement, mut transform) in &mut query_movement {
        let distance_to_move = movement.speed * time.delta_seconds() * time_scale.0;
        move_to_target_location(&mut movement, &mut transform, distance_to_move);
        if movement.path.is_empty() {
            movement.to_idle(entity, &mut commands);
        }
    }
}

pub fn move_to_target_location(
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
