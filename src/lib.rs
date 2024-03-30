pub use bevy::prelude::*;
pub use bevy_asset_loader::prelude::*;
pub use bevy_inspector_egui::prelude::*;

pub mod assets;
pub mod camera;
pub mod input;
pub mod map;
pub mod movement;
pub mod navigation;
pub mod pawn;
pub mod settings;
pub mod story_time;
pub mod structure;
pub mod ui;
pub mod utils;

pub use crate::assets::*;
pub use crate::camera::*;
pub use crate::input::*;
pub use crate::map::*;
pub use crate::movement::*;
pub use crate::navigation::*;
pub use crate::pawn::*;
pub use crate::settings::*;
pub use crate::story_time::*;
pub use crate::ui::*;
pub use crate::utils::*;

#[derive(Debug, Event)]
pub struct StateChangeEvent<T>(pub T);

#[derive(Debug, Event)]
pub struct EntityStateChangeEvent<T>(pub Entity, pub T);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum WorldState {
    #[default]
    Loading,
    Playing,
}
