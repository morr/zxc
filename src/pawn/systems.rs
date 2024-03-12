use super::components::*;
use crate::configs;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn spawn_pawns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning pawns");

    let mesh = Mesh::from(Circle::new(0.5));
    let material = ColorMaterial::from(Color::hex("E178C5").unwrap());
    let mesh_handle: Handle<Mesh> = meshes.add(mesh);
    let material_handle = materials.add(material);

    for i in 1..configs::STARTING_PAWNS {
        commands.spawn((PawnBundle {
            structure: Pawn {},
            name: Name::new("Base"),
            mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: material_handle.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        },));
    }
}
