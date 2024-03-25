use std::collections::HashMap;

use bevy::prelude::*;
use pathfinding::prelude::astar;

use crate::{
    map::components::ClickTileEvent,
    pawn::{Pawn, PawnStatus},
    utils::TranslationHelper,
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
                start: transform.translation.truncate().world_pos_to_tile(),
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
        println!("{:?}", event);

        let path = astar(
            &event.start,
            |&IVec2 { x, y }| {
                let left = (x - 1, y); // look at saturating_add/sub
                let top = (x, y - 1);
                let right = (x + 1, y); // look at saturating_add/sub
                let bottom = (x, y + 1);

                [left, top, right, bottom]
                    .iter()
                    .filter_map(|&(x, y)| {
                        navmesh
                            .get_if_passable(x, y)
                            .map(|navtile| (IVec2 { x, y }, navtile.cost))
                    })
                    .collect::<Vec<_>>()
            },
            // try (distance_x + distance_y) / 3 as it is suggested in docs
            // https://docs.rs/pathfinding/latest/pathfinding/directed/astar/fn.astar.html
            |&pos| {
                (Vec2::new(pos.x as f32, pos.y as f32)
                    - Vec2::new(event.end.x as f32, event.end.y as f32))
                .length() as i32
            },
            |&pos| pos == event.end,
        )
            .map(|(vec, _cost)| vec);

        pathfind_event_writer.send(PathfindAnswerEvent {
            entity: event.entity,
            start: event.start,
            end: event.end,
            path,
        });
    }
}

pub fn listen_for_pathfinding_answers(
    mut pathfind_event_reader: EventReader<PathfindAnswerEvent>,
) {
    for event in pathfind_event_reader.read() {
        println!("{:?}", event);

        if let Some(path) = &event.path {
        } else {
            error!("pathfind failed {:?}", event);
        }
    }
}
