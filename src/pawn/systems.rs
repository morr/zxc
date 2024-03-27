use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;

use self::structure::{Structure, BASE_HEIGHT, BASE_WIDTH};

use super::*;

pub fn spawn_pawns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // q: Query<(&Structure, &Transform)>,
    // q: Query<(Entity, &GlobalTransform), (With<Structure>)>,
    // q: Query<(Entity, &GlobalTransform), (With<Structure>)>,
    // q: Query<&Transform, With<Structure>>,
    // q: Query<&Name, With<Structure>>,
    query: Query<&Transform, With<Structure>>,
    // mut q: Query<&Structure>,
    // query_base: Query<&Structure>,
) {
    // println!("Spawning pawns");

    let mesh = Mesh::from(Circle::new(TILE_SIZE / 2.0));
    let material = ColorMaterial::from(Color::hex("E178C5").unwrap());
    let mesh_handle: Handle<Mesh> = meshes.add(mesh);
    let material_handle = materials.add(material);

    let mut rng = rand::thread_rng();
    let radius = TILE_SIZE * f32::max(BASE_WIDTH, BASE_HEIGHT);

    let transform = query.single();

    for i in 0..STARTING_PAWNS {
        let random_angle: f32 = rng.gen_range(0.0..360.0);
        let x = transform.translation.x + random_angle.cos() * radius;
        let y = transform.translation.y + random_angle.sin() * radius;

        commands
            .spawn(PawnBundle {
                pawn: Pawn::default(),
                name: Name::new(format!("Pawn {i}")),
                status: PawnStatus::Idle,
                mesh_bundle: MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    material: material_handle.clone(),
                    transform: Transform::from_xyz(
                        grid_tile_center_to_world(world_pos_to_grid(x)),
                        grid_tile_center_to_world(world_pos_to_grid(y)),
                        // world_pos_to_tile_aligned(x),
                        // world_pos_to_tile(x),
                        // world_pos_to_tile_aligned(y),
                        // world_pos_to_tile(y),
                        PAWN_Z_INDEX,
                    ),
                    ..default()
                },
            })
            .insert(ShowAabbGizmo {
                color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
            });
        // .insert((Actor, Pathing::default()));
        // .insert(RigidBody::Dynamic)
        // .insert(Collider::rectangle(1.0, 1.0))
        // .insert(CollisionLayers::new([Layer::Actor], [Layer::Terrain]));
    }
}

pub fn move_pawns(
    mut query_pawns: Query<(Entity, &mut Pawn, &mut Transform), With<Pawn>>,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
) {
    for (_entity, mut pawn, mut transform) in &mut query_pawns {
        move_to_target_location(
            &mut pawn,
            &mut transform,
            PAWN_SPEED * time.delta_seconds() * time_scale.0,
        );
    }
}

pub fn move_to_target_location(pawn: &mut Pawn, transform: &mut Transform, distance_to_move: f32) {
    let current_point_world: Vec2 = transform.translation.truncate();

    if let Some(target_point_tile) = pawn.move_path.front() {
        let target_point_world = target_point_tile.grid_tile_center_to_world();
        let direction = (target_point_world - current_point_world).normalize_or_zero();
        let distance_between_points = (target_point_world - current_point_world).length();

        if distance_to_move >= distance_between_points {
            transform.translation = target_point_world.extend(transform.translation.z);
            pawn.move_path.pop_front();

            let remaining_distance = distance_to_move - distance_between_points;

            if remaining_distance > 0.0 {
                move_to_target_location(pawn, transform, remaining_distance);
            }
        } else {
            transform.translation += (direction * distance_to_move).extend(0.0);
        }
    }
}

// pub fn wander_pawns(
//     time: Res<Time>,
//     mut query: Query<(&mut Transform, &mut Pawn, &Name), With<Pawn>>,
//     time_scale: Res<TimeScale>,
// ) {
//     let mut rng = rand::thread_rng();
//
//     for (mut transform, mut pawn, _name) in &mut query {
//         pawn.retry_pathfinding_timer.tick(time.delta());
//
//         if pawn.retry_pathfinding_timer.finished() {
//             pawn.retry_pathfinding_timer = Timer::new(
//                 Duration::from_secs_f32(rng.gen_range(0.5..3.0)),
//                 TimerMode::Once,
//             );
//             let random_angle: f32 = rng.gen_range(0.0..360.0);
//             pawn.move_path = Some(Vec2::new(random_angle.cos(), random_angle.sin()));
//         }
//
//         if let Some(move_vector) = pawn.move_vector {
//             transform.translation.x +=
//                 move_vector.x * PAWN_SPEED * time.delta_seconds() * time_scale.0;
//             transform.translation.y +=
//                 move_vector.y * PAWN_SPEED * time.delta_seconds() * time_scale.0;
//         }
//     }
// }
