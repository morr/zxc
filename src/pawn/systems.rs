use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use rand_distr::{num_traits::Zero, Distribution, UnitCircle};

use self::structure::{Farm, Warehouse, BASE_HEIGHT, BASE_WIDTH, FARM_TILE_SIZE};

use super::*;

const MAX_ATTEMPTS_TO_FIND_IDLE_WALK_PATH: usize = 10;

#[allow(clippy::too_many_arguments)]
pub fn spawn_pawns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets_collection: Res<AssetsCollection>,
    font_assets: Res<FontAssets>,
    warehouse_query: Query<&Transform, With<Warehouse>>,
    farm_query: Query<&Transform, With<Farm>>,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut occupation_change_event_writer: EventWriter<OccupationChangeEvent>,
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

    let mut navmesh = arc_navmesh.write();
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

        let pawn_id = commands
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
            // .insert(ShowAabbGizmo {
            //     color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
            // })
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
            })
            .id();

        let grid_tile = position.truncate().world_pos_to_grid();
        navmesh.add_occupation::<Movable>(pawn_id, grid_tile.x, grid_tile.y);
        occupation_change_event_writer.send(OccupationChangeEvent::new(grid_tile));
    }
}

pub fn update_pawn_color(
    assets_collection: Res<AssetsCollection>,
    mut movable_event_reader: EventReader<EntityStateChangeEvent<MovableState>>,
    mut pawn_event_reader: EventReader<EntityStateChangeEvent<PawnState>>,
    mut query: Query<&mut Handle<ColorMaterial>>,
) {
    for event in movable_event_reader.read() {
        if let Ok(mut material_handle) = query.get_mut(event.0) {
            *material_handle = match event.1 {
                MovableState::Idle => assets_collection.pawn_idle.clone(),
                MovableState::Moving => assets_collection.pawn_moving.clone(),
                MovableState::Pathfinding(_end_tile) => assets_collection.pawn_pathfinding.clone(),
                MovableState::PathfindingError => assets_collection.pawn_pathfinding_error.clone(),
            };
        }
    }

    for event in pawn_event_reader.read() {
        if let Ok(mut material_handle) = query.get_mut(event.0) {
            match event.1 {
                PawnState::Working(_) => *material_handle = assets_collection.pawn_working.clone(),
                PawnState::Dead => *material_handle = assets_collection.pawn_dead.clone(),
                _ => {}
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
    mut query: Query<(Entity, &mut Pawn), Without<pawn_state::Dead>>,
) {
    for event in event_reader.read() {
        for (entity, mut pawn) in query.iter_mut() {
            if pawn.is_birthday(event.0) {
                pawn.age += 1;
                // event_writer.send(PawnBirthdayEvent(entity));
            }
            pawn.decrease_lifetime(CONFIG.time.day_duration);

            if pawn.lifetime <= CONFIG.time.day_duration {
                commands.entity(entity).insert(DyingMarker);
            }
        }
    }
}

pub fn progress_pawn_dying(
    mut commands: Commands,
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &mut Pawn), With<DyingMarker>>,
    mut event_writer: EventWriter<PawnDeathEvent>,
) {
    for (entity, mut pawn) in query.iter_mut() {
        pawn.decrease_lifetime(time_scale.scale_to_seconds(time.delta_seconds()));

        if pawn.lifetime.is_zero() {
            event_writer.send(PawnDeathEvent(entity));
            commands.entity(entity).remove::<DyingMarker>();
        }
    }
}

pub fn progress_pawn_death(
    mut commands: Commands,
    mut event_reader: EventReader<PawnDeathEvent>,
    mut query: Query<&mut Pawn>,
    mut state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
    mut work_queue: ResMut<TasksQueue>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);

        let entity = event.0;
        let mut pawn = query.get_mut(event.0).unwrap();

        // set pawn dead
        let prev_state = pawn.change_state(
            PawnState::Dead,
            entity,
            &mut commands,
            &mut state_change_event_writer,
        );

        // return pawn task back to tasks queue
        if let PawnState::WorkAssigned(task) | PawnState::Working(task) = prev_state {
            work_queue.add_task(task);
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
