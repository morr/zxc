use super::*;

pub mod pawn_status {
    use bevy::prelude::*; // Assuming you're using Bevy for the `Component` derive

    macro_rules! define_pawn_statuses {
        // Match a tuple of tuples, with the first one treated as default
        (($first_enum_name:ident, $first_component_name:ident) $(, ($enum_name:ident, $component_name:ident))*) => {
            #[derive(Debug, Default)] // Use the standard Default derive
            pub enum PawnStatus {
                #[default] // This marks the first variant as the default.
                $first_enum_name,
                $($enum_name),*
            }

            #[derive(Component)]
            pub struct $first_component_name;

            $(
                #[derive(Component)]
                pub struct $component_name;
            )*
        };
    }

    // Use the macro with the new tuple of pairs format
    define_pawn_statuses!((Idle, PawnIdle), (Moving, PawnMoving));
}

#[derive(Component, Default)]
pub struct Pawn {
    // pub status: pawn_status::PawnStatus,
    // pub age: u32,
    // pub retry_pathfinding_timer: Timer,
}

// #[derive(Bundle)]
// pub struct PawnBundle {
//     pub pawn: Pawn,
//     pub name: Name,
//     pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
//     pub movement: Movement,
//     // pub movement_bundle: MovementBundle
//     // pub status: PawnStatus,
// }
