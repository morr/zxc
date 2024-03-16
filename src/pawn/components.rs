use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Pawn {
    pub move_vector: Option<Vec2>,
    pub retry_pathfinding_timer: Timer,
}

#[derive(Bundle)]
pub struct PawnBundle {
    pub pawn: Pawn,
    pub name: Name,
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}
