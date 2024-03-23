use bevy::prelude::*;
use bevy_flowfield_tiles_plugin::bundle::FlowFieldTilesBundle;

use super::*;
use crate::{GRID_COLS, GRID_ROWS};

pub fn setup_navigation(mut commands: Commands) {
    let map_length = GRID_COLS;
    let map_depth = GRID_ROWS;
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
    mut actor_q: Query<&mut Pathing, With<Actor>>,
) {
    for event in event_reader.read() {
        let mut pathing = actor_q.get_single_mut().unwrap();

        pathing.target_position = Some(event.0);
        pathing.metadata = None;
        pathing.portal_route = None;
        pathing.has_los = false;
    }
}
