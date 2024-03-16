use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Pawn {
    // pub age: u32,
    pub move_vector: Option<Vec2>,
    pub retry_pathfinding_timer: Timer,
}

#[derive(Bundle)]
pub struct PawnBundle {
    pub pawn: Pawn,
    pub name: Name,
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

// impl Default for Pawn {
//     fn default() -> Self {
//         Self {
//             wave: 0,
//             enemy_count_multiplier: 1,
//             enemy_spawn_timer: Timer::from_seconds(30.0, TimerMode::Repeating),
//         }
//     }
// }
