pub use bevy::prelude::*;
pub use bevy_asset_loader::prelude::*;
pub use bevy_inspector_egui::prelude::*;
pub use std::sync::LazyLock;
// pub use bevy_magic_light_2d::prelude::*;

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

#[derive(Event, Debug)]
pub struct StateChangeEvent<T>(pub T);

#[derive(EntityEvent, Debug)]
pub struct EntityStateChangeEvent<T>(#[event_target] pub Entity, pub T);

#[derive(States, Default, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppState {
    #[default]
    Loading,
    Playing,
    Quiting,
}

#[macro_export]
macro_rules! ensure_state {
    // For loops - uses continue
    (loop: $expected_pattern:pat, $current_state:expr) => {
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
    (loop: $expected_state:expr, $current_state:expr) => {
        if $current_state != $expected_state {
            trace!(
                "Got {:?} while expected {:?} by Query<With<_>> param",
                $current_state, $expected_state,
            );
            continue;
        }
    };

    // For functions - uses return
    (fn: $expected_pattern:pat, $current_state:expr) => {
        match $current_state {
            $expected_pattern => {}
            _ => {
                trace!(
                    "Got {:?} while expected pattern {:?} by Query<With<_>> param",
                    $current_state,
                    stringify!($expected_pattern),
                );
                return;
            }
        }
    };
    (fn: $expected_state:expr, $current_state:expr) => {
        if $current_state != $expected_state {
            trace!(
                "Got {:?} while expected {:?} by Query<With<_>> param",
                $current_state, $expected_state,
            );
            return;
        }
    };
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
macro_rules! return_unless {
    ($expected_pattern:pat, $current_state:expr) => {
        match $current_state {
            $expected_pattern => {}
            _ => {
                return;
            }
        }
    };
    ($expected_state:expr, $current_state:expr) => {
        if $current_state != $expected_state {
            return;
        }
    };
}

/// Generates unit tag components for a state machine group where at most one
/// tag may be present on an entity at a time. Inserting any tag of the group
/// removes its siblings via an `on_add` component hook, so the invariant holds
/// at the ECS level no matter who inserts the tag. Tags are `immutable`.
#[macro_export]
macro_rules! exclusive_state_tags {
    ($($tag:ident),+ $(,)?) => {
        $crate::exclusive_state_tags!(@step [] | $($tag),+);
    };
    (@step [$($before:ident)*] | $cur:ident $(, $rest:ident)*) => {
        #[derive(bevy::prelude::Component, Debug, Default, bevy::prelude::Reflect)]
        #[component(immutable, on_add = $cur::remove_sibling_tags)]
        pub struct $cur;

        impl $cur {
            fn remove_sibling_tags(
                mut world: bevy::ecs::world::DeferredWorld,
                ctx: bevy::ecs::lifecycle::HookContext,
            ) {
                world
                    .commands()
                    .entity(ctx.entity)
                    .remove::<($($before,)* $($rest,)*)>();
            }
        }

        $crate::exclusive_state_tags!(@step [$($before)* $cur] | $($rest),*);
    };
    (@step [$($before:ident)*] |) => {};
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
