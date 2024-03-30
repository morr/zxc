use pathfinding::directed::astar::astar;

use super::*;

use crate::{map::components::ClickTileEvent, utils::WorldTranslationHelper};

pub fn pathfinding_on_click(
    mut commands: Commands,
    mut click_event_reader: EventReader<ClickTileEvent>,
    mut query_pawns: Query<(Entity, &Transform, &mut Movement), With<Movement>>,
    mut pathfind_event_writer: EventWriter<PathfindRequestEvent>,
    mut movement_state_event_writer: EventWriter<EntityStateChangeEvent<MovementState>>,
) {
    for click_event in click_event_reader.read() {
        for (entity, transform, mut movement) in &mut query_pawns {
            movement.to_pathfinding(
                entity,
                transform.translation.truncate().world_pos_to_grid(),
                click_event.0,
                &mut commands,
                &mut pathfind_event_writer,
                &mut movement_state_event_writer,
            );
        }
    }
}

pub fn listen_for_pathfinding_requests(
    navmesh: Res<Navmesh>,
    mut pathfind_event_reader: EventReader<PathfindRequestEvent>,
    mut pathfind_event_writer: EventWriter<PathfindAnswerEvent>,
) {
    for event in pathfind_event_reader.read() {
        // println!("{:?}", event);

        let path = if navmesh.get_if_passable(event.end.x, event.end.y).is_some() {
            astar(
                &event.start,
                |&IVec2 { x, y }| {
                    [
                        (x - 1, y),     // left
                        (x - 1, y - 1), // left-top
                        (x, y - 1),     // top
                        (x + 1, y - 1), // top-right
                        (x + 1, y),     // right
                        (x + 1, y + 1), // right-bototm
                        (x, y + 1),     // bottom
                        (x - 1, y + 1), // bottom-left
                    ]
                    .iter()
                    .filter_map(|&(nx, ny)| {
                        navmesh.get_if_passable(nx, ny).and_then(|navtile| {
                            let is_diagonal_movement = x != nx && y != ny;

                            if !is_diagonal_movement
                            // check that both adjacent tiles are passable
                            || (navmesh.get_if_passable(x, ny).is_some()
                                && navmesh.get_if_passable(nx, y).is_some())
                            {
                                Some((
                                    IVec2 { x: nx, y: ny },
                                    if is_diagonal_movement {
                                        // this is not strictly correct calculation
                                        // instead of cost * sqrt(2) it should be
                                        // (tile1.cost + sqrt(2))/2 + (tile2.cost + sqrt(2))/2
                                        (navtile.cost as f32 * f32::sqrt(2.0)).floor() as i32
                                    } else {
                                        navtile.cost
                                    },
                                ))
                            } else {
                                None
                            }
                        })
                    })
                    .collect::<Vec<_>>()
                },
                // try (distance_x + distance_y) / 3 as it is suggested in docs
                // https://docs.rs/pathfinding/latest/pathfinding/directed/astar/fn.astar.html
                |&pos| {
                    let length = (Vec2::new(pos.x as f32, pos.y as f32)
                        - Vec2::new(event.end.x as f32, event.end.y as f32))
                    .length();

                    // println!("{} {}", length, (length * COST_MULTIPLIER) as i32);
                    (length * COST_MULTIPLIER) as i32
                },
                |&pos| pos == event.end,
            )
            .map(|(vec, _cost)| vec)
        } else {
            None
        };

        // if path.is_none() {
        //     error!("PathfindingError {:?}", event);
        // }

        pathfind_event_writer.send(PathfindAnswerEvent {
            entity: event.entity,
            start: event.start,
            end: event.end,
            path,
        });
    }
}

pub fn listen_for_pathfinding_answers(
    mut commands: Commands,
    mut pathfind_event_reader: EventReader<PathfindAnswerEvent>,
    mut query_movement: Query<(Entity, &mut Movement), With<Movement>>,
    mut movement_state_event_writer: EventWriter<EntityStateChangeEvent<MovementState>>,
) {
    for event in pathfind_event_reader.read() {
        // println!("{:?}", event);

        let Ok((entity, mut movement)) = query_movement.get_mut(event.entity) else {
            continue;
        };
        if let MovementState::Pathfinding(end_tile) = movement.state {
            // check if it an is outdated pathfinding answer
            if end_tile != event.end {
                // println!(
                //     "end_tile != event.end, end_tile={}, event.end={}",
                //     end_tile, event.end
                // );
                return;
            }

            if let Some(path) = &event.path {
                if path.len() == 1 {
                    movement.to_idle(entity, &mut commands, &mut movement_state_event_writer);
                } else {
                    movement.to_moving(
                        path.iter().skip(1).cloned().collect(),
                        entity,
                        &mut commands,
                        &mut movement_state_event_writer,
                    );
                }
            } else {
                movement.to_pathfinding_error(entity, &mut movement_state_event_writer);
            }
        } else {
            println!(
                "movement.state != MovementState::Pathfinding, movement.state={:?}",
                movement.state
            );
        }
    }
}
