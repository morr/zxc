use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use std::time::Duration;

use super::*;

#[derive(Component)]
pub struct Pawn {
    pub age: u32,
    pub retry_pathfinding_timer: Timer,
}

impl Default for Pawn {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            age: rng.gen_range(14..32),
            retry_pathfinding_timer: Timer::new(Duration::from_secs(0), TimerMode::Once),
        }
    }
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum PawnStatus {
    #[default]
    Idle,
    Pathfinding,
    PathfindingError,
    Moving,
}

#[derive(Bundle)]
pub struct PawnBundle {
    pub pawn: Pawn,
    pub name: Name,
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub status: PawnStatus,
    pub movement: Movement
}
