use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use rand_distr::{num_traits::Zero, Distribution, UnitCircle};

use super::*;

const MAX_ATTEMPTS_TO_FIND_IDLE_WALK_PATH: usize = 10;

#[allow(clippy::too_many_arguments)]
pub fn spawn_pawns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets_collection: Res<AssetsCollection>,
    // font_assets: Res<FontAssets>,
    warehouse_query: Query<&Transform, With<Warehouse>>,
    farm_query: Query<&Transform, With<Farm>>,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut occupation_change_event_writer: EventWriter<OccupationChangeEvent>,
    mut user_selection_command_writer: EventWriter<UserSelectionCommand>,
) {
    // println!("Spawning pawns");

    let mesh = Mesh::from(Circle::new(CONFIG.tile.size / 2.0 * 0.75));
    // let material = ColorMaterial::from(Color::hex("E178C5").unwrap());
    let mesh_handle: Handle<Mesh> = meshes.add(mesh);
    // let material_handle = materials.add(material);

    let mut rng = rand::thread_rng();
    let radius = CONFIG.tile.size * i32::max(BASE_WIDTH, BASE_HEIGHT) as f32;

    let warehouse_transform = warehouse_query.single();
    let maybe_farm_transform = farm_query.iter().next();

    let mut navmesh = arc_navmesh.write();
    for i in 0..CONFIG.starting_scene.pawns {
        let random_angle: f32 = rng.gen_range(0.0..360.0);

        let position = if i >= 4 || maybe_farm_transform.is_none() {
            Vec3::new(
                warehouse_transform.translation.x + random_angle.cos() * radius,
                warehouse_transform.translation.y + random_angle.sin() * radius,
                PAWN_Z_INDEX,
            )
        } else {
            let farm_transform = maybe_farm_transform.unwrap();
            Vec3::new(
                farm_transform.translation.x
                    + random_angle.cos() * 5.0 * FARM_TILE_SIZE as f32 * CONFIG.tile.size,
                farm_transform.translation.y
                    + random_angle.sin() * 5.0 * FARM_TILE_SIZE as f32 * CONFIG.tile.size,
                PAWN_Z_INDEX,
            )
        };
        let pawn = Pawn::default();
        // let pawn_state_string = format!("{:?}", pawn.state);

        let pawn_id = commands
            .spawn((
                pawn,
                pawn_state::PawnStateIdleTag,
                Commandable::default(),
                Name::new("Pawn"),
                // state: PawnState::Idle,
                MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    material: assets_collection.pawn_idle.clone(),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                Movable::new(CONFIG.pawn.speed * CONFIG.tile.size),
                Restable::default(),
            ))
            // .insert(ShowAabbGizmo {
            //     color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
            // })
            // .with_children(|parent| {
            //     parent.spawn((
            //         Text2dBundle {
            //             text: Text::from_section(
            //                 pawn_state_string,
            //                 TextStyle {
            //                     font: font_assets.fira.clone(),
            //                     font_size: 13.0,
            //                     color: Color::WHITE,
            //                 },
            //             ),
            //             transform: Transform::from_xyz(0.0, 21.0, PAWN_Z_INDEX),
            //             ..default()
            //         },
            //         PawnStateText,
            //     ));
            // })
            .id();

        let grid_tile = position.truncate().world_pos_to_grid();
        navmesh.add_occupation::<Movable>(pawn_id, grid_tile.x, grid_tile.y);
        occupation_change_event_writer.send(OccupationChangeEvent::new(grid_tile));

        // auto-select first pawn
        if i.is_zero() {
            user_selection_command_writer.send(UserSelectionCommand(Some(UserSelectionData {
                entity: pawn_id,
                kind: UserSelectionKind::Pawn,
            })));
        }
    }
}

pub fn update_pawn_color(// assets_collection: Res<AssetsCollection>,
    // mut movable_event_reader: EventReader<EntityStateChangeEvent<MovableState>>,
    // mut pawn_event_reader: EventReader<EntityStateChangeEvent<PawnState>>,
    // mut query: Query<&mut Handle<ColorMaterial>>,
) {
    // for event in movable_event_reader.read() {
    //     if let Ok(mut material_handle) = query.get_mut(event.0) {
    //         *material_handle = match event.1 {
    //             MovableState::Idle => assets_collection.pawn_idle.clone(),
    //             MovableState::Moving => assets_collection.pawn_moving.clone(),
    //             MovableState::Pathfinding(_end_tile) => assets_collection.pawn_pathfinding.clone(),
    //             MovableState::PathfindingError => assets_collection.pawn_pathfinding_error.clone(),
    //         };
    //     }
    // }
    //
    // for event in pawn_event_reader.read() {
    //     if let Ok(mut material_handle) = query.get_mut(event.0) {
    //         match event.1 {
    //             PawnState::Working(_) => *material_handle = assets_collection.pawn_working.clone(),
    //             PawnState::Dead => *material_handle = assets_collection.pawn_dead.clone(),
    //             _ => {}
    //         };
    //     }
    // }
}

pub fn wander_idle_pawns(
    mut commands: Commands,
    arc_navmesh: Res<ArcNavmesh>,
    mut query: Query<
        (Entity, &Pawn, &Movable, &mut Commandable, &Transform),
        (
            With<pawn_state::PawnStateIdleTag>,
            With<commandable_state::CommandableStateIdleTag>,
        ),
    >,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
) {
    let mut rng = rand::thread_rng();

    for (commandable_entity, pawn, movable, mut commandable, transform) in &mut query {
        ensure_state!(PawnState::Idle, pawn.state);
        ensure_state!(CommandableState::Idle, commandable.state);
        if movable.state != MovableState::Idle {
            continue;
        }

        let world_pos = transform.translation.truncate();
        let end_tile = find_valid_end_tile(world_pos, &arc_navmesh.read(), &mut rng, 0);

        commandable.set_queue(
            CommandType::MoveTo(MoveToCommand { commandable_entity, grid_tile: end_tile}),
            commandable_entity,
            &mut commands,
            &mut tasks_scheduler,
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

pub fn update_pawn_state_text(// mut event_reader: EventReader<EntityStateChangeEvent<PawnState>>,
    // children_query: Query<&Children>,
    // mut state_text_query: Query<&mut Text, With<PawnStateText>>,
) {
    // for EntityStateChangeEvent(id, state) in event_reader.read() {
    //     // println!("{:?}", event);
    //     for text_entity in children_query.iter_descendants(*id) {
    //         let mut text = state_text_query.get_mut(text_entity).unwrap();
    //         text.sections[0].value = match state {
    //             PawnState::Working(_) => "Working".into(),
    //             // PawnState::TaskAsigned() => format!("state: {:?}", StateDebug(state)),
    //             _ => format!("{:?}", state),
    //         };
    //     }
    // }
}

pub fn progress_pawn_daily(
    mut commands: Commands,
    mut event_reader: EventReader<NewDayEvent>,
    // mut event_writer: EventWriter<PawnBirthdayEvent>,
    mut query: Query<(Entity, &mut Pawn), Without<pawn_state::PawnStateDeadTag>>,
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
    mut query: Query<(&mut Pawn, &mut Commandable)>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
) {
    for PawnDeathEvent(entity) in event_reader.read() {
        // println!("{:?}", PawnDeathEvent(pawn_entity));

        match query.get_mut(*entity) {
            Ok((mut pawn, mut commandable)) => {
                pawn.change_state(
                    PawnState::Dead,
                    *entity,
                    &mut commands,
                    // &mut state_change_event_writer,
                );

                commandable.clear_queue(*entity, &mut commands, &mut tasks_scheduler);
            }
            Err(err) => {
                warn!("Failed to get query result: {:?}", err);
                continue;
            }
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
