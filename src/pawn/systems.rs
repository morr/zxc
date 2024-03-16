use std::time::Duration;

use super::components::*;
use crate::settings::Settings;
use crate::PAWN_SPEED;
use crate::TILE_SIZE;
use crate::{configs, structure::Structure, utils::TranslationHelper};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::prelude::*;

pub fn spawn_pawns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // q: Query<(&Structure, &Transform)>,
    // q: Query<(Entity, &GlobalTransform), (With<Structure>)>,
    // q: Query<(Entity, &GlobalTransform), (With<Structure>)>,
    // q: Query<&Transform, With<Structure>>,
    // q: Query<&Name, With<Structure>>,
    q: Query<&Transform, With<Structure>>,
    // mut q: Query<&Structure>,

    // query_base: Query<&Structure>,
) {
    println!("Spawning pawns");

    let mesh = Mesh::from(Circle::new(0.5));
    let material = ColorMaterial::from(Color::hex("E178C5").unwrap());
    let mesh_handle: Handle<Mesh> = meshes.add(mesh);
    let material_handle = materials.add(material);

    let mut rng = rand::thread_rng();
    let radius = TILE_SIZE * 2.0;

    let transform = q.single();

    for i in 0..configs::STARTING_PAWNS {
        let random_angle: f32 = rng.gen_range(0.0..360.0);
        let pos = Vec2::new(
            transform.translation.x + random_angle.cos() * radius,
            transform.translation.y + random_angle.sin() * radius,
        )
        .world_pos_to_tile()
        .tile_pos_to_world();

        commands.spawn((PawnBundle {
            pawn: Pawn {
                move_vector: None,
                retry_pathfinding_timer: Timer::new(Duration::from_secs(0), TimerMode::Once),
            },
            name: Name::new(format!("Pawn {i}")),
            mesh_bundle: MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                material: material_handle.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, 0.0),
                ..default()
            },
        },));
    }
}

pub fn wander_pawns(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut Pawn, &Name), With<Pawn>>,
    settings: Res<Settings>,
) {
    let mut rng = rand::thread_rng();

    for (mut transform, mut pawn, _name) in &mut q {
        pawn.retry_pathfinding_timer.tick(time.delta());

        if pawn.retry_pathfinding_timer.finished() {
            pawn.retry_pathfinding_timer = Timer::new(
                Duration::from_secs_f32(rng.gen_range(0.5..3.0)),
                TimerMode::Once,
            );
            let random_angle: f32 = rng.gen_range(0.0..360.0);
            pawn.move_vector = Some(Vec2::new(random_angle.cos(), random_angle.sin()));
        }

        if let Some(move_vector) = pawn.move_vector {
            transform.translation.x +=
                move_vector.x * PAWN_SPEED * time.delta_seconds() * settings.time_scale;
            transform.translation.y +=
                move_vector.y * PAWN_SPEED * time.delta_seconds() * settings.time_scale;
        }
    }
}
