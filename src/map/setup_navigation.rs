use bevy::prelude::*;
use bevy_flowfield_tiles_plugin::{
    bundle::FlowFieldTilesBundle,
    flowfields::{fields::RouteCache, sectors::MapDimensions},
    plugin::flow_layer::EventPathRequest,
};

use super::*;
use crate::{utils::TranslationHelper, GRID_COLS, GRID_ROWS};

pub fn setup_navigation(mut commands: Commands) {
    let map_length = GRID_COLS as u32;
    let map_depth = GRID_ROWS as u32;
    let sector_resolution = 25;
    let actor_size = 1.0;

    commands.spawn(FlowFieldTilesBundle::new(
        map_length,
        map_depth,
        sector_resolution,
        actor_size,
    ));
}

pub fn pathfinding_on_click(
    mut event_reader: EventReader<ClickTileEvent>,
    dimensions_q: Query<&MapDimensions>,
    mut actor_q: Query<&mut Pathing, With<Actor>>,
) {
    for event in event_reader.read() {
        let map_dimensions = dimensions_q.get_single().unwrap();

        if map_dimensions
            .get_sector_and_field_cell_from_xy(Vec2::new(event.0.x as f32, event.0.y as f32))
            .is_some()
        {
            let mut pathing = actor_q.get_single_mut().unwrap();

            pathing.target_position = Some(event.0);
            pathing.metadata = None;
            pathing.portal_route = None;
            pathing.has_los = false;
        }
    }
}

pub fn get_or_request_route(
    route_q: Query<(&RouteCache, &MapDimensions)>,
    mut actor_q: Query<(&Transform, &mut Pathing), With<Actor>>,
    mut event: EventWriter<EventPathRequest>,
) {
    let (route_cahe, map_dimensions) = route_q.get_single().unwrap();
    for (tform, mut pathing) in &mut actor_q {
        if let Some(target) = pathing.target_position {
            // actor has no route, look one up or request one
            if pathing.portal_route.is_none() {
                if let Some((source_sector, source_field)) = map_dimensions
                    .get_sector_and_field_cell_from_xy(
                        tform.translation.truncate().world_pos_to_tile(),
                    )
                {
                    if let Some((target_sector, goal_id)) =
                        map_dimensions.get_sector_and_field_cell_from_xy(target)
                    {
                        // if a route is calculated get it
                        if let Some((metadata, route)) = route_cahe.get_route_with_metadata(
                            source_sector,
                            source_field,
                            target_sector,
                            goal_id,
                        ) {
                            pathing.metadata = Some(*metadata);
                            pathing.portal_route = Some(route.clone());
                            // println!("{:?}", pathing);
                        } else {
                            // request a route
                            event.send(EventPathRequest::new(
                                source_sector,
                                source_field,
                                target_sector,
                                goal_id,
                            ));
                        }
                    }
                }
            }
        }
    }
}
