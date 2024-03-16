use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;
use std::time::Duration;

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

impl Default for Pawn {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            // age: rng.gen_range(14..32),
            move_vector: None,
            retry_pathfinding_timer: Timer::new(Duration::from_secs(0), TimerMode::Once),
        }
    }
}

// impl Pawn {
//     fn new() -> Self {
//         Pawn {
//         }
//     }
// }
