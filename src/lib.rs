pub use bevy::prelude::*;
pub use bevy_asset_loader::prelude::*;
pub use bevy_inspector_egui::prelude::*;
pub use std::sync::LazyLock;
// pub use bevy_magic_light_2d::prelude::*;

pub mod events;
mod macros;
pub mod state;

// ВРЕМЕННО: снимается в 4.5 вместе с остальными плоскими реэкспортами
pub use crate::{events::*, state::*};

pub mod ai;
pub mod assets;
pub mod async_queue;
pub mod camera;
pub mod carryable;
pub mod commandable;
pub mod config;
pub mod daylight;
pub mod feedable;
pub mod input;
pub mod map;
pub mod movable;
pub mod navigation;
pub mod pawn;
pub mod restable;
pub mod story_time;
pub mod structure;
pub mod tasks_queue;
pub mod ui;
pub mod user_selection;
pub mod workable;

// ВРЕМЕННО: плоские реэкспорты, снимаются в 4.5
pub use crate::{
    ai::*, assets::*, async_queue::*, camera::*, carryable::*, commandable::*, config::*,
    daylight::*, feedable::*, input::*, map::*, movable::*, navigation::*, pawn::*, restable::*,
    story_time::*, structure::*, tasks_queue::*, ui::*, user_selection::*, workable::*,
};
