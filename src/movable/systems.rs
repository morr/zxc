use super::*;

pub fn move_moving_entities(
    mut commands: Commands,
    mut query_movable: Query<
        (Entity, &mut Movable, &mut Transform, Option<&Pawn>),
        With<MovableStateMovinTag>,
    >,
    time: Res<Time>,
    arc_navmesh: Res<ArcNavmesh>,
    mut occupation_change_event_writer: MessageWriter<OccupationChangeMessage>,
) {
    for (entity, mut movable, mut transform, maybe_pawn) in &mut query_movable {
        match movable.state {
            MovableState::Moving(_) => {
                let current_tile = transform.translation.truncate().world_pos_to_grid();
                let final_tile = move_to_target_location(
                    entity,
                    &mut movable,
                    &mut transform,
                    time.delta_secs(),
                    &arc_navmesh,
                    &mut commands,
                );

                if current_tile != final_tile {
                    let mut navmesh = arc_navmesh.write();

                    if maybe_pawn.is_some() {
                        navmesh.remove_occupant::<Pawn>(&entity, current_tile.x, current_tile.y);
                        navmesh.add_occupant::<Pawn>(&entity, final_tile.x, final_tile.y);
                    }

                    occupation_change_event_writer
                        .write(OccupationChangeMessage(vec![current_tile, final_tile]));
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
) -> IVec2 {
    if movable.path.is_empty() {
        let current_tile = transform.translation.truncate().world_pos_to_grid();
        let is_destination_reached = match movable.state {
            MovableState::Moving(target_tile)
            // | MovableState::Pathfinding(target_tile)
            // | MovableState::PathfindingError(target_tile)
                if current_tile == target_tile =>
            {
                true
            }
            _ => false,
        };
        // println!(
        //     "MessageWriter<MovableReachedDestinationEvent> current_tile:{:?}, target_tile:{:?}",
        //     current_tile,
        //     match movable.state {
        //         MovableState::Moving(target_tile) => Some(target_tile),
        //         _ => None,
        //     }
        // );
        movable.to_idle(entity, commands, is_destination_reached);
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
            );
        }
    } else {
        transform.translation += (direction * distance_to_move).extend(0.0);
    }

    transform.translation.truncate().world_pos_to_grid()
}

pub fn on_pawn_death(
    event: On<PawnDeatEvent>,
    mut commands: Commands,
    mut query: Query<&mut Movable>,
) {
    let PawnDeatEvent { entity, .. } = *event;

    let Ok(mut movable) = query.get_mut(entity) else {
        return;
    };

    movable.to_idle(entity, &mut commands, false);
}
