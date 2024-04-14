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


// macro_rules! define_pawn_statuses {
//     // This pattern matches each enum variant and its corresponding structure component.
//     ($(($name:ident, $struct_name:ident)),*) => {
//         #[derive(Debug)]
//         pub enum PawnStatus {
//             $($name),*
//         }
//
//         $(
//             #[derive(Component)]
//             pub struct $struct_name;
//         )*
//     };
// }
//
// // Use the macro to define both the enum and the corresponding structure components.
// define_buildings_and_structures!(
//     (Base, BaseStructure),
//     (Home, HomeStructure),
//     (Warehouse, WarehouseStructure),
//     (Farm, FarmStructure)
// );

#[derive(Bundle)]
pub struct PawnBundle {
    pub pawn: Pawn,
    pub name: Name,
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub movement: Movement,
    // pub movement_bundle: MovementBundle
    // pub status: PawnStatus,
}
