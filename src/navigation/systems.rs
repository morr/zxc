use bevy::prelude::*;

use crate::map::components::ClickTileEvent;

use super::*;

pub fn generate_navmesh() {}

pub fn pathfinding_on_click(
    mut event_reader: EventReader<ClickTileEvent>,
    // dimensions_q: Query<&MapDimensions>,
    // mut actor_q: Query<&mut Pathing, With<Actor>>,
) {
    for event in event_reader.read() {
        println!("{:?}", event);
    }
}

pub fn listen_for_pathfinding_requests(mut event_reader: EventReader<PathfindingRequestEvent>) {
    for event in event_reader.read() {
        println!("{:?}", event);
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
