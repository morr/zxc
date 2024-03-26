use bevy::prelude::*;
use pathfinding::prelude::astar;

use crate::{
    map::components::ClickTileEvent,
    pawn::{Pawn, PawnStatus},
    utils::WorldTranslationHelper,
};

use super::*;

pub fn generate_navmesh() {}

pub fn pathfinding_on_click(
    mut click_event_reader: EventReader<ClickTileEvent>,
    mut query_pawns: Query<(Entity, &Transform, &mut PawnStatus), With<Pawn>>,
    mut pathfind_event_writer: EventWriter<PathfindRequestEvent>,
    // dimensions_q: Query<&MapDimensions>,
    // mut actor_q: Query<&mut Pathing, With<Actor>>,
) {
    for click_event in click_event_reader.read() {
        for (entity, transform, mut pawn_status) in &mut query_pawns {
            *pawn_status = PawnStatus::Pathfinding;

            pathfind_event_writer.send(PathfindRequestEvent {
                start: transform.translation.truncate().world_pos_to_grid(),
                end: click_event.0,
                entity,
            });
        }
    }
}

pub fn listen_for_pathfinding_requests(
    navmesh: Res<NavMesh>,
    mut pathfind_event_reader: EventReader<PathfindRequestEvent>,
    mut pathfind_event_writer: EventWriter<PathfindAnswerEvent>,
) {
    for event in pathfind_event_reader.read() {
        // println!("{:?}", event);

        let path = astar(
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
        .map(|(vec, _cost)| vec);

        if path.is_none() {
            error!("PathfindingError {:?}", event);
        }

        pathfind_event_writer.send(PathfindAnswerEvent {
            entity: event.entity,
            path,
        });
    }
}

pub fn listen_for_pathfinding_answers(
    mut pathfind_event_reader: EventReader<PathfindAnswerEvent>,
    mut q_pawns: Query<(&mut Pawn, &mut PawnStatus), With<Pawn>>,
) {
    for event in pathfind_event_reader.read() {
        // println!("{:?}", event);

        let Ok((mut pawn, mut pawn_status)) = q_pawns.get_mut(event.entity) else {
            continue;
        };

        if let Some(path) = &event.path {
            if path.len() == 1 {
                *pawn_status = PawnStatus::Idle;
            } else {
                pawn.move_path = path.iter().skip(1).cloned().collect();
                *pawn_status = PawnStatus::Moving;
            }
        } else {
            *pawn_status = PawnStatus::PathfindingError;
        }
    }
}
