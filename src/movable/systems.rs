use super::*;

pub fn move_moving_entities(
    mut commands: Commands,
    mut query_movable: Query<(Entity, &mut Movable, &mut Transform), With<MovableMoving>>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    arc_navmesh: Res<ArcNavmesh>,
    mut movable_state_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
    mut occupation_change_event_writer: EventWriter<OccupationChangeEvent>,
) {
    for (entity, mut movable, mut transform) in &mut query_movable {
        let current_tile = transform.translation.truncate().world_pos_to_grid();
        let final_tile = move_to_target_location(
            entity,
            &mut movable,
            &mut transform,
            time_scale.scale_to_seconds(time.delta_seconds()),
            &arc_navmesh,
            &mut commands,
            &mut movable_state_event_writer,
        );

        if current_tile != final_tile {
            let mut navmesh = arc_navmesh.write();

            navmesh.remove_occupation::<Movable>(&entity, current_tile.x, current_tile.y);
            navmesh.add_occupation::<Movable>(entity, final_tile.x, final_tile.y);

            occupation_change_event_writer
                .send(OccupationChangeEvent(vec![current_tile, final_tile]));
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
    event_writer: &mut EventWriter<EntityStateChangeEvent<MovableState>>,
) -> IVec2 {
    if movable.path.is_empty() {
        movable.to_idle(entity, commands, Some(event_writer));
        return transform.translation.truncate().world_pos_to_grid();
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
    for event in event_reader.read() {
        let entity = event.0;
        let mut movable = query.get_mut(event.0).unwrap();
        // println!("{:?}", event);
        movable.to_idle(entity, &mut commands, None);
    }
}
