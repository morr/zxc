use bevy::prelude::*;

use crate::{map::components::ClickTileEvent, pawn::{Pawn, PawnStatus}, utils::TranslationHelper};

use super::*;

pub fn generate_navmesh() {}

pub fn pathfinding_on_click(
    mut click_event_reader: EventReader<ClickTileEvent>,
    mut query_pawns: Query<(Entity, &Transform, &mut PawnStatus), (With<Pawn>)>,
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
    navmesh: Res<Navmesh>,
    mut pathfind_event_reader: EventReader<PathfindRequestEvent>,
    mut pathfind_event_writer: EventWriter<PathfindAnswerEvent>,

) {
    if pathfind_event_reader.is_empty() { return; }

    let navmesh = &navmesh.0;

    for request in pathfind_event_reader.read() {
        println!("{:?}", request);

        let Vec2 { x, y } = request.start;
        let x = x as usize;
        let y = y as usize;

        let Vec2 { x: end_x, y: end_y } = request.end;
        let end_x = end_x as usize;
        let end_y = end_y as usize;

        let result = astar(
            &UsizeVec { x, y },
            |&UsizeVec { x, y }| {
                let up = (x, y.saturating_add(1));
                let down = (x, y.saturating_sub(1));
                let left = (x.saturating_sub(1), y);
                let right = (x.saturating_add(1), y);

                let neighbors = [up, down, left, right]
                    .iter()
                    .filter(|&(x, y)| {
                        navmesh
                            .get(*x)
                            .and_hen(|row| row.get(*y))
                            .map(|tile| tile.walkable || (*x == end_x && *y == end_y))
                            .unwrap_or(false)
                    })
                    .map(|(x, y)| (UsizeVec { x: *x, y: *y }, 0)) // Modify this line
                    .collect::<Vec<_>>();

                neighbors
            },
            |&tile| {
                (Vec2::new(tile.x as f32, tile.y as f32) - Vec2::new(end_x as f32, end_y as f32))
                    .length() as i32
            },
            |UsizeVec { x, y }| x == &end_x && y == &end_y,
        )
        .map(|(data, _)| {
            data.iter()
                .map(|item| Vec2::new(item.x as f32, item.y as f32))
                .collect::<Vec<_>>()
        });

        pathfinding_event_writer.send(PathfindAnswer {
            path: result,
            entity: request.entity,
            target: request.end,
        });
t

    }
}

// use bevy::prelude::*;
// use bevy_flowfield_tiles_plugin::{
//     bundle::FlowFieldTilesBundle,
//     flowfields::{
//         fields::{
//             flow_field::{get_2d_direction_unit_vector_from_bits, has_line_of_sight},
//             Field, FlowFieldCache, RouteCache,
//         },
//         sectors::MapDimensions,
//     },
//     plugin::flow_layer::EventPathRequest,
// };
// use bevy_xpbd_2d::components::LinearVelocity;
//
// use super::*;
// use crate::{utils::TranslationHelper, GRID_COLS, GRID_ROWS};
//
// pub fn setup_navigation(mut commands: Commands) {
//     let map_length = GRID_COLS as u32;
//     let map_depth = GRID_ROWS as u32;
//     let sector_resolution = 25;
//     let actor_size = 1.0;
//
//     commands.spawn(FlowFieldTilesBundle::new(
//         map_length,
//         map_depth,
//         sector_resolution,
//         actor_size,
//     ));
// }
//
// pub fn pathfinding_on_click(
//     mut event_reader: EventReader<ClickTileEvent>,
//     dimensions_q: Query<&MapDimensions>,
//     mut actor_q: Query<&mut Pathing, With<Actor>>,
// ) {
//     for event in event_reader.read() {
//         let map_dimensions = dimensions_q.get_single().unwrap();
//
//         if map_dimensions
//             .get_sector_and_field_cell_from_xy(Vec2::new(event.0.x as f32, event.0.y as f32))
//             .is_some()
//         {
//             let mut pathing = actor_q.get_single_mut().unwrap();
//
//             pathing.target_position = Some(event.0);
//             pathing.metadata = None;
//             pathing.portal_route = None;
//             pathing.has_los = false;
//         }
//     }
// }
//
// pub fn get_or_request_route(
//     route_q: Query<(&RouteCache, &MapDimensions)>,
//     mut actor_q: Query<(&Transform, &mut Pathing), With<Actor>>,
//     mut event: EventWriter<EventPathRequest>,
// ) {
//     let (route_cahe, map_dimensions) = route_q.get_single().unwrap();
//     for (tform, mut pathing) in &mut actor_q {
//         if let Some(target) = pathing.target_position {
//             // actor has no route, look one up or request one
//             if pathing.portal_route.is_none() {
//                 if let Some((source_sector, source_field)) = map_dimensions
//                     .get_sector_and_field_cell_from_xy(
//                         tform.translation.truncate().world_pos_to_tile(),
//                     )
//                 {
//                     if let Some((target_sector, goal_id)) =
//                         map_dimensions.get_sector_and_field_cell_from_xy(target)
//                     {
//                         // if a route is calculated get it
//                         if let Some((metadata, route)) = route_cahe.get_route_with_metadata(
//                             source_sector,
//                             source_field,
//                             target_sector,
//                             goal_id,
//                         ) {
//                             pathing.metadata = Some(*metadata);
//                             pathing.portal_route = Some(route.clone());
//                         } else {
//                             // request a route
//                             event.send(EventPathRequest::new(
//                                 source_sector,
//                                 source_field,
//                                 target_sector,
//                                 goal_id,
//                             ));
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
//
// const SPEED: f32 = 64.0;
// pub fn actor_steering(
//     mut actor_q: Query<(&mut LinearVelocity, &mut Transform, &mut Pathing), With<Actor>>,
//     flow_cache_q: Query<(&FlowFieldCache, &MapDimensions)>,
//     time_step: Res<Time>,
// ) {
//     let (flow_cache, map_dimensions) = flow_cache_q.get_single().unwrap();
//     for (mut velocity, tform, mut pathing) in actor_q.iter_mut() {
//         // lookup the overarching route
//         if let Some(route) = pathing.portal_route.as_mut() {
//             // find the current actors postion in grid space
//             if let Some((curr_actor_sector, curr_actor_field_cell)) =
//                 map_dimensions.get_sector_and_field_cell_from_xy(tform.translation.truncate())
//             {
//                 // trim the actor stored route as it makes progress
//                 // this ensures it doesn't use a previous goal from
//                 // a sector it has already been through when it needs
//                 // to pass through it again as part of a different part of the route
//                 if let Some(f) = route.first() {
//                     if curr_actor_sector != f.0 {
//                         route.remove(0);
//                     }
//                 }
//                 // lookup the relevant sector-goal of this sector
//                 'routes: for (sector, goal) in route.iter() {
//                     if *sector == curr_actor_sector {
//                         // get the flow field
//                         if let Some(field) = flow_cache.get_field(*sector, *goal) {
//                             // based on actor field cell find the directional vector it should move in
//                             let cell_value = field.get_field_cell_value(curr_actor_field_cell);
//                             if has_line_of_sight(cell_value) {
//                                 pathing.has_los = true;
//                                 let dir =
//                                     pathing.target_position.unwrap() - tform.translation.truncate();
//                                 velocity.0 = dir.normalize() * SPEED * time_step.delta_seconds();
//                                 break 'routes;
//                             }
//                             let dir = get_2d_direction_unit_vector_from_bits(cell_value);
//                             if dir.x == 0.0 && dir.y == 0.0 {
//                                 warn!("Stuck");
//                                 pathing.portal_route = None;
//                             }
//                             velocity.0 = dir * SPEED * time_step.delta_seconds();
//                         }
//                         break 'routes;
//                     }
//                 }
//             }
//         }
//     }
// }
