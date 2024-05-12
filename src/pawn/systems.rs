use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use rand_distr::{Distribution, UnitCircle};

use self::structure::{Farm, Warehouse, BASE_HEIGHT, BASE_WIDTH, FARM_TILE_SIZE};

use super::*;

const MAX_ATTEMPTS_TO_FIND_IDLE_WALK_PATH: usize = 10;

pub fn spawn_pawns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets_collection: Res<AssetsCollection>,
    font_assets: Res<FontAssets>,
    warehouse_query: Query<&Transform, With<Warehouse>>,
    farm_query: Query<&Transform, With<Farm>>,
    // query: Query<&Transform, With<Warehouse>>,
) {
    // println!("Spawning pawns");

    let mesh = Mesh::from(Circle::new(CONFIG.tile.size / 2.0 * 0.75));
    // let material = ColorMaterial::from(Color::hex("E178C5").unwrap());
    let mesh_handle: Handle<Mesh> = meshes.add(mesh);
    // let material_handle = materials.add(material);

    let mut rng = rand::thread_rng();
    let radius = CONFIG.tile.size * i32::max(BASE_WIDTH, BASE_HEIGHT) as f32;

    let warehouse_transform = warehouse_query.single();
    let farm_transform = farm_query.iter().next().unwrap();

    for i in 0..CONFIG.starting_scene.pawns {
        let random_angle: f32 = rng.gen_range(0.0..360.0);

        let position = if i >= 4 {
            Vec3::new(
                warehouse_transform.translation.x + random_angle.cos() * radius,
                warehouse_transform.translation.y + random_angle.sin() * radius,
                PAWN_Z_INDEX,
            )
        } else {
            Vec3::new(
                farm_transform.translation.x
                    + random_angle.cos() * 5.0 * FARM_TILE_SIZE as f32 * CONFIG.tile.size,
                farm_transform.translation.y
                    + random_angle.sin() * 5.0 * FARM_TILE_SIZE as f32 * CONFIG.tile.size,
                PAWN_Z_INDEX,
            )
        };
        let pawn = Pawn::default();
        let pawn_state_string = format!("{:?}", pawn.state);

        commands
            .spawn((
                pawn,
                pawn_state::Idle,
                Name::new("Pawn"),
                // state: PawnState::Idle,
                MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    material: assets_collection.pawn_idle.clone(),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                Movable::new(CONFIG.pawn.speed * CONFIG.tile.size),
            ))
            .insert(ShowAabbGizmo {
                color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
            })
            .with_children(|parent| {
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            pawn_state_string,
                            TextStyle {
                                font: font_assets.fira.clone(),
                                font_size: 13.0,
                                color: Color::WHITE,
                            },
                        ),
                        transform: Transform::from_xyz(0.0, 21.0, PAWN_Z_INDEX),
                        ..default()
                    },
                    PawnStateText,
                ));
            });
    }
}

pub fn update_pawn_color(
    assets_collection: Res<AssetsCollection>,
    mut event_reader: EventReader<EntityStateChangeEvent<MovableState>>,
    mut query: Query<&mut Handle<ColorMaterial>>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);

        if let Ok(mut material_handle) = query.get_mut(event.0) {
            *material_handle = match event.1 {
                MovableState::Idle => assets_collection.pawn_idle.clone(),
                MovableState::Moving => assets_collection.pawn_moving.clone(),
                MovableState::Pathfinding(_end_tile) => assets_collection.pawn_pathfinding.clone(),
                MovableState::PathfindingError => assets_collection.pawn_pathfinding_error.clone(),
            };
        }
    }
}

pub fn wander_idle_pawns(
    arc_navmesh: Res<ArcNavmesh>,
    queue_counter: Res<AsyncQueueCounter>,
    mut commands: Commands,
    // time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut Movable,
            &Transform,
            Option<&mut PathfindingTask>,
        ),
        With<pawn_state::Idle>,
    >,
    // time_scale: Res<TimeScale>,
    // mut pathfind_event_writer: EventWriter<PathfindRequestEvent>,
    mut movable_state_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
) {
    let mut rng = rand::thread_rng();

    for (entity, mut movable, transform, mut maybe_pathfinding_task) in &mut query {
        if movable.state != MovableState::Idle {
            continue;
        }

        let world_pos = transform.translation.truncate();
        let start_tile = world_pos.world_pos_to_grid();
        let end_tile = find_valid_end_tile(world_pos, &arc_navmesh.read(), &mut rng, 0);

        // movable.to_pathfinding(
        //     entity,
        //     start_tile,
        //     end_tile,
        //     &mut commands,
        //     &mut pathfind_event_writer,
        //     &mut movable_state_event_writer,
        // );
        movable.to_pathfinding_async(
            entity,
            start_tile,
            end_tile,
            &arc_navmesh,
            &queue_counter,
            maybe_pathfinding_task.as_deref_mut(),
            &mut commands,
            &mut movable_state_event_writer,
        );
    }
}

fn find_valid_end_tile(
    start_pos: Vec2,
    navmesh: &Navmesh,
    rng: &mut impl Rng,
    recursion_depth: usize,
) -> IVec2 {
    let move_vector: Vec2 = UnitCircle.sample(rng).into();
    let tiles_to_move = rng.gen_range(3.0..12.0) * CONFIG.tile.size;
    let end_tile = (start_pos + move_vector * tiles_to_move).world_pos_to_grid();

    if recursion_depth >= MAX_ATTEMPTS_TO_FIND_IDLE_WALK_PATH {
        return end_tile;
    }

    if navmesh.is_passable(end_tile.x, end_tile.y) {
        end_tile
    } else {
        let offsets = [
            IVec2::new(-1, -1), // left-top
            IVec2::new(0, -1),  // top
            IVec2::new(1, -1),  // right-top
            IVec2::new(-1, 0),  // left
            IVec2::new(1, 0),   // right
            IVec2::new(-1, 1),  // left-bottom
            IVec2::new(0, 1),   // bottom
            IVec2::new(1, 1),   // right-bottom
        ];

        offsets
            .iter()
            .map(|offset| end_tile + *offset)
            .find(|&tile| navmesh.is_passable(tile.x, tile.y))
            .unwrap_or_else(|| find_valid_end_tile(start_pos, navmesh, rng, recursion_depth + 1))
    }
}

pub fn update_pawn_state_text(
    mut event_reader: EventReader<EntityStateChangeEvent<PawnState>>,
    children_query: Query<&Children>,
    mut state_text_query: Query<&mut Text, With<PawnStateText>>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);
        for text_entity in children_query.iter_descendants(event.0) {
            let mut text = state_text_query.get_mut(text_entity).unwrap();
            text.sections[0].value = format!("{:?}", event.1);
        }
    }
}

pub fn progress_pawn_daily(
    mut commands: Commands,
    mut event_reader: EventReader<NewDayEvent>,
    // mut event_writer: EventWriter<PawnBirthdayEvent>,
    mut query: Query<(Entity, &mut Pawn)>,
) {
    for event in event_reader.read() {
        for (entity, mut pawn) in query.iter_mut() {
            if pawn.is_birthday(event.0) {
                pawn.age += 1;
                // event_writer.send(PawnBirthdayEvent(entity));
            }
            pawn.lifetime -= CONFIG.time.day_duration;

            if pawn.lifetime <= CONFIG.time.day_duration {
                commands.entity(entity).insert(Dying);
            }
        }
    }
}

pub fn progress_pawn_dyuing(
    mut commands: Commands,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut Pawn), With<Dying>>,
    mut state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for (entity, mut pawn) in query.iter_mut() {
        pawn.lifetime -= time_scale.scale_to_seconds(time.delta_seconds());

        if pawn.lifetime <= 0.0 {
            pawn.change_state(
                PawnState::Dead,
                entity,
                &mut commands,
                &mut state_change_event_writer,
            );
        }
    }
}

// pub fn progress_pawn_age(
//     mut event_reader: EventReader<PawnBirthdayEvent>,
//     mut query: Query<&mut Pawn>,
// ) {
//     for event in event_reader.read() {
//         if let Ok(mut pawn) = query.get_mut(event.0) {
//             pawn.age += 1;
//         }
//     }
// }
//
// pub fn progress_pawn_lifetime(
//     mut event_reader: EventReader<NewDayEvent>,
//     mut query: Query<&mut Pawn>,
// ) {
//     for event in event_reader.read() {
//         println!("{:?}", event);
//         for mut pawn in query.iter_mut() {
//             pawn.lifetime -= CONFIG.time.day_duration;
//         }
//     }
// }
