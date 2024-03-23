use bevy::prelude::*;
use bevy_flowfield_tiles_plugin::bundle::FlowFieldTilesBundle;

use crate::{GRID_COLS, GRID_ROWS};

// use super::*;

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
