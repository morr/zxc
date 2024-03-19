use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub const BASE_WIDTH: f32 = 8.0;
pub const BASE_HEIGHT: f32 = 12.0;

#[derive(Component)]
pub struct Structure {}

#[derive(Bundle)]
pub struct StructureBundle {
    pub structure: Structure,
    pub name: Name,
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}
