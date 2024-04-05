use std::collections::VecDeque;

use crate::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Movement>()
            .add_systems(Update, apply_movement.run_if(in_state(TimeState::Running)));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub enum MovementState {
    #[default]
    Idle,
    Moving,
    Pathfinding(IVec2),
    PathfindingError,
}

#[derive(Component)]
pub struct MovementMoving;

#[derive(Debug, Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Movement {
    pub path: VecDeque<IVec2>,
    pub speed: f32,
    pub state: MovementState,
}

impl Movement {
    pub fn new(speed: f32) -> Self {
        Self {
            path: VecDeque::new(),
            speed,
            state: MovementState::Idle,
        }
    }

    pub fn to_idle(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        movement_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovementState>>,
    ) {
        self.stop_moving(entity, commands);
        self.state = MovementState::Idle;
        movement_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
    }

    pub fn to_moving(
        &mut self,
        path: VecDeque<IVec2>,
        entity: Entity,
        commands: &mut Commands,
        movement_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovementState>>,
    ) {
        self.state = MovementState::Moving;
        self.path = path;
        commands.entity(entity).insert(MovementMoving);
        movement_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn to_pathfinding_async(
        &mut self,
        entity: Entity,
        start_tile: IVec2,
        end_tile: IVec2,
        arc_navmesh: &Res<ArcNavmesh>,
        queue_counter: &Res<AsyncQueueCounter>,
        commands: &mut Commands,
        movement_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovementState>>,
    ) {
        if self.state == MovementState::Moving {
            self.stop_moving(entity, commands);
        }
        self.state = MovementState::Pathfinding(end_tile);
        movement_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));

        // queue_counter.increment();
        // let queue_counter_clone = queue_counter.0.clone();

        let navmesh_arc_clone = arc_navmesh.0.clone();
        let task = spawn_task(queue_counter, async move {
            // queue_counter_clone.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            let navmesh = navmesh_arc_clone.read().unwrap();

            PathfindingResult {
                start_tile,
                end_tile,
                path: astar_pathfinding(&navmesh, &start_tile, &end_tile),
            }
        });

        // if let Ok(mut existing_task) = commands.entity(entity).get_mut::<PathfindingTask>() {
        //     existing_task.add(task);
        // } else {
        //     commands.entity(entity).insert(PathfindingTask::new(task));
        // }
    }

    // pub fn to_pathfinding(
    //     &mut self,
    //     entity: Entity,
    //     start_tile: IVec2,
    //     end_tile: IVec2,
    //     commands: &mut Commands,
    //     pathfind_event_writer: &mut EventWriter<PathfindRequestEvent>,
    //     movement_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovementState>>,
    // ) {
    //     if self.state == MovementState::Moving {
    //         self.stop_moving(entity, commands);
    //     }
    //
    //     self.state = MovementState::Pathfinding(end_tile);
    //     pathfind_event_writer.send(PathfindRequestEvent {
    //         start_tile,
    //         end_tile,
    //         entity,
    //     });
    //     movement_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
    // }

    pub fn to_pathfinding_error(
        &mut self,
        entity: Entity,
        movement_state_event_writer: &mut EventWriter<EntityStateChangeEvent<MovementState>>,
    ) {
        self.state = MovementState::PathfindingError;
        self.path = [].into();
        movement_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
    }

    fn stop_moving(&mut self, entity: Entity, commands: &mut Commands) {
        self.path = [].into();
        commands.entity(entity).remove::<MovementMoving>();
    }
}

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
