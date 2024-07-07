pub use bevy::prelude::*;
pub use bevy_asset_loader::prelude::*;
pub use bevy_inspector_egui::prelude::*;
pub use once_cell::sync::Lazy;
// pub use bevy_magic_light_2d::prelude::*;

macro_rules! expose_submodules {
    ( $( $x:ident ),* ) => {
        $(
            mod $x;
            pub use self::$x::*;
        )*
    };
}

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

pub use crate::ai::*;
pub use crate::assets::*;
pub use crate::async_queue::*;
pub use crate::camera::*;
pub use crate::carryable::*;
pub use crate::commandable::*;
pub use crate::config::*;
pub use crate::daylight::*;
pub use crate::feedable::*;
pub use crate::input::*;
pub use crate::map::*;
pub use crate::movable::*;
pub use crate::navigation::*;
pub use crate::pawn::*;
pub use crate::restable::*;
pub use crate::story_time::*;
pub use crate::structure::*;
pub use crate::tasks_queue::*;
pub use crate::ui::*;
pub use crate::user_selection::*;
pub use crate::workable::*;

#[derive(Debug, Event)]
pub struct StateChangeEvent<T>(pub T);

#[derive(Debug, Event)]
pub struct EntityStateChangeEvent<T>(pub Entity, pub T);

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppState {
    #[default]
    Loading,
    Playing,
    Quiting,
}

#[macro_export]
macro_rules! ensure_state {
    ($expected_pattern:pat, $current_state:expr) => {
        match $current_state {
            $expected_pattern => {}
            _ => {
                trace!(
                    "Got {:?} while expected pattern {:?} by Query<With<_>> param",
                    $current_state,
                    stringify!($expected_pattern),
                );
                continue;
            }
        }
    };
    ($expected_state:expr, $current_state:expr) => {
        if $current_state != $expected_state {
            trace!(
                "Got {:?} while expected {:?} by Query<With<_>> param",
                $current_state,
                $expected_state,
            );
            continue;
        }
   # };
}

#[macro_export]
macro_rules! continue_unless {
    ($expected_pattern:pat, $current_state:expr) => {
        match $current_state {
            $expected_pattern => {}
            _ => {
                continue;
            }
        }
    };
    ($expected_state:expr, $current_state:expr) => {
        if $current_state != $expected_state {
            continue;
        }
    };
}

#[macro_export]
macro_rules! log_state_change {
    ($($arg:tt)+) => {
        debug!($($arg)+);
    };
}

#[macro_export]
macro_rules! log_event {
    ($event:expr) => {{
        let event = $event;
        debug!("Event {:?}", &event);
        event
    }};
}
