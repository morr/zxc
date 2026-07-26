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

macro_rules! use_modules {
    ( $( $x:ident ),* ) => {
        $(
            pub mod $x;
            pub use crate::$x::*;
        )*
    };
}

macro_rules! expose_submodules {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
            pub use self::$x::*;
        )*
    };
}

use_modules!(
    ai,
    assets,
    async_queue,
    camera,
    carryable,
    commandable,
    config,
    daylight,
    feedable,
    input,
    map,
    movable,
    navigation,
    pawn,
    restable,
    story_time,
    structure,
    tasks_queue,
    ui,
    user_selection,
    workable
);
