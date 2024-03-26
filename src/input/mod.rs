use crate::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PrevHoveredTilePos>()
            .add_systems(Update, mouse_input);
    }
}
