use bevy::sprite::MaterialMesh2dBundle;

use super::*;

#[derive(Component, Default)]
pub struct Pawn {
    // pub age: u32,
    // pub retry_pathfinding_timer: Timer,
}

// impl Default for Pawn {
//     fn default() -> Self {
//         // let mut rng = rand::thread_rng();
//
//         Self {
//             // age: rng.gen_range(14..32),
//             // retry_pathfinding_timer: Timer::new(Duration::from_secs(0), TimerMode::Once),
//         }
//     }
// }

// #[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Default, States)]
// pub enum PawnStatus {
//     #[default]
//     Idle,
//     Pathfinding,
//     PathfindingError,
//     Moving,
// }


pub mod pawn_status {
    use super::*;

    macro_rules! define_pawn_statuses {
        // This pattern matches each enum variant and its corresponding structure component.
        ($(($enum_name:ident, $component_name:ident)),*) => {
            #[derive(Debug)]
            pub enum PawnStatus {
                $($enum_name),*
            }

            $(
                #[derive(Component)]
                pub struct $component_name;
            )*
        };
    }
    //
    // // Use the macro to define both the enum and the corresponding structure components.
    define_pawn_statuses!(
        (Idle, PawnIdle),
        (Moving, PawnMoving)
    );
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
