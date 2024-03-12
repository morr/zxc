use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

#[derive(Component)]
pub struct Structure {}

#[derive(Bundle)]
pub struct StructureBundle {
    pub structure: Structure,
    pub name: Name,
    pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
}
