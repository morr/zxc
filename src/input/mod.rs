use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

use crate::{camera::MainCamera, map::components::HoverTileEvent, utils::world_pos_to_tile};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PrevHoveredTilePos>()
            .add_systems(Update, smoothened_mouse_movement);
    }
}
