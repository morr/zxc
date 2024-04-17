pub use bevy::prelude::*;
pub use bevy_asset_loader::prelude::*;
pub use bevy_inspector_egui::prelude::*;
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
pub mod camera;
pub mod daylight;
pub mod input;
pub mod map;
pub mod movement;
pub mod navigation;
pub mod pawn;
pub mod settings;
pub mod story_time;
pub mod structure;
pub mod tasks_queues;
pub mod ui;
pub mod utils;

pub use crate::assets::*;
pub use crate::camera::*;
pub use crate::daylight::*;
pub use crate::input::*;
pub use crate::map::*;
pub use crate::movement::*;
pub use crate::navigation::*;
pub use crate::pawn::*;
pub use crate::settings::*;
pub use crate::story_time::*;
pub use crate::tasks_queues::*;
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
