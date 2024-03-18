use crate::TILE_SIZE;

use super::components::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn spawn_base(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning base");

    let mesh = Mesh::from(Rectangle::new(TILE_SIZE * 2.0, TILE_SIZE * 2.0));
    let material = ColorMaterial::from(Color::rgb(1., 0., 0.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((StructureBundle {
        structure: Structure {},
        name: Name::new("Base"),
        mesh_bundle: MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    },));
}
