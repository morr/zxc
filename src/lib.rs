pub use bevy::prelude::*;
pub use bevy_asset_loader::prelude::*;
pub use bevy_inspector_egui::prelude::*;
use kayak_ui::prelude::{widgets::*, *};
// pub use bevy_magic_light_2d::prelude::*;

macro_rules! expose_submodules {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
            pub use self::$x::*;
        )*
    };
}

pub mod assets;
pub mod async_queue;
pub mod camera;
pub mod config;
pub mod daylight;
pub mod input;
pub mod item;
pub mod map;
pub mod movable;
pub mod navigation;
pub mod pawn;
pub mod story_time;
pub mod structure;
pub mod ui;
pub mod workable;

pub use crate::assets::*;
pub use crate::async_queue::*;
pub use crate::camera::*;
pub use crate::config::*;
pub use crate::daylight::*;
pub use crate::input::*;
pub use crate::item::*;
pub use crate::map::*;
pub use crate::movable::*;
pub use crate::navigation::*;
pub use crate::pawn::*;
pub use crate::story_time::*;
pub use crate::ui::*;
pub use crate::workable::*;

#[derive(Debug, Event)]
pub struct StateChangeEvent<T>(pub T);

#[derive(Debug, Event)]
pub struct EntityStateChangeEvent<T>(pub Entity, pub T);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    Loading,
    Playing,
}
