use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Pawn {}

#[derive(Bundle)]
pub struct PawnBundle {
    pub structure: Pawn,
    pub name: Name,
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}
