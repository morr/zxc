use rand::Rng;
use rand_distr::num_traits::Zero;

use super::*;

#[allow(clippy::too_many_arguments)]
pub fn spawn_pawns(
    mut commands: Commands,
    assets_collection: Res<AssetsCollection>,
    meshes_collection: Res<MeshesCollection>,
    font_assets: Res<FontAssets>,
    // warehouse_query: Query<&Transform, With<Warehouse>>,
    farm_query: Query<&Transform, With<Farm>>,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut occupation_change_event_writer: EventWriter<OccupationChangeEvent>,
    // mut user_selection_command_writer: EventWriter<UserSelectionCommand>,
) {
    let mut rng = rand::rng();
    let radius = config().tile.size * i32::max(BASE_WIDTH, BASE_HEIGHT) as f32;

    // let warehouse_transform = warehouse_query.single();
    let maybe_farm_transform = farm_query.iter().next();

    let mut navmesh = arc_navmesh.write();
    for i in 0..config().starting_scene.pawns {
        let random_angle: f32 = rng.random_range(0.0..360.0);

        let position = if i >= 4 || maybe_farm_transform.is_none() {
            Vec3::new(
                random_angle.cos() * radius,
                random_angle.sin() * radius,
                // warehouse_transform.translation.x + random_angle.cos() * radius,
                // warehouse_transform.translation.y + random_angle.sin() * radius,
                PAWN_Z_INDEX,
            )
        } else {
            let farm_transform = maybe_farm_transform.unwrap();
            Vec3::new(
                farm_transform.translation.x
                    + random_angle.cos() * 5.0 * FARM_TILE_SIZE as f32 * config().tile.size,
                farm_transform.translation.y
                    + random_angle.sin() * 5.0 * FARM_TILE_SIZE as f32 * config().tile.size,
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
                Mesh2d(meshes_collection.pawn.clone()),
                MeshMaterial2d(assets_collection.pawn_idle.clone()),
                Transform::from_translation(position),
                Movable::new(config().pawn.speed * config().tile.size),
                Restable::default(),
                Feedable::default(),
            ))
            // .insert(ShowAabbGizmo {
            //     color: Some(Color::srgba(1.0, 1.0, 1.0, 0.25)),
            // })
            .with_children(|parent| {
                parent.spawn((
                    Text::new(""),
                    TextFont {
                        font: font_assets.fira.clone(),
                        font_size: 13.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, 21.0, PAWN_Z_INDEX),
                    PawnStateText,
                ));
            })
            .id();

        let grid_tile = position.truncate().world_pos_to_grid();
        navmesh.add_occupant::<Pawn>(&pawn_id, grid_tile.x, grid_tile.y);
        occupation_change_event_writer.send(log_event!(OccupationChangeEvent::new(grid_tile)));

        // auto-select first pawn
        // if i.is_zero() {
        //     user_selection_command_writer.send(log_event!(UserSelectionCommand(Some(
        //         UserSelectionData {
        //             entity: pawn_id,
        //             kind: UserSelectionKind::Pawn,
        //         }
        //     ))));
        // }
    }
}

// pub fn update_pawn_color(// assets_collection: Res<AssetsCollection>,
//     // mut movable_event_reader: EventReader<EntityStateChangeEvent<MovableState>>,
//     // mut pawn_event_reader: EventReader<EntityStateChangeEvent<PawnState>>,
//     // mut query: Query<&mut Handle<ColorMaterial>>,
// ) {
//     // for event in movable_event_reader.read() {
//     //     if let Ok(mut material_handle) = query.get_mut(event.0) {
//     //         *material_handle = match event.1 {
//     //             MovableState::Idle => assets_collection.pawn_idle.clone(),
//     //             MovableState::Moving => assets_collection.pawn_moving.clone(),
//     //             MovableState::Pathfinding(_end_tile) => assets_collection.pawn_pathfinding.clone(),
//     //             MovableState::PathfindingError => assets_collection.pawn_pathfinding_error.clone(),
//     //         };
//     //     }
//     // }
//     //
//     // for event in pawn_event_reader.read() {
//     //     if let Ok(mut material_handle) = query.get_mut(event.0) {
//     //         match event.1 {
//     //             PawnState::Working(_) => *material_handle = assets_collection.pawn_working.clone(),
//     //             PawnState::Dead => *material_handle = assets_collection.pawn_dead.clone(),
//     //             _ => {}
//     //         };
//     //     }
//     // }
// }

pub fn update_pawn_state_text(
    mut event_reader: EventReader<EntityStateChangeEvent<PawnState>>,
    children_query: Query<&Children>,
    // mut state_text_query: Query<(&mut Text, &mut Visibility), With<PawnStateText>>,
    // mut state_text_query: Query<&mut Text, With<PawnStateText>>,
    mut text_writer: TextUiWriter,
    commandable_query: Query<&Commandable>,
) {
    for EntityStateChangeEvent(pawn_entity, state) in event_reader.read() {
        // println!("{:?}", event);
        for text_entity in children_query.iter_descendants(*pawn_entity) {
            // let (mut text, mut visibility) = state_text_query.get_mut(text_entity).unwrap();
            // let mut text = state_text_query.get_mut(text_entity).unwrap();

            // *visibility = Visibility::Visible;

            *text_writer.text(text_entity, 0) = match state {
                PawnState::Idle => "Idle".into(),
                PawnState::Dead => "DEAD".into(),
                PawnState::ExecutingCommand => {
                    let commandable = commandable_query.get(*pawn_entity).unwrap();
                    if let Some(command_type) = &commandable.executing {
                        (match command_type {
                            // CommandType::MoveTo(_) => "",
                            CommandType::Sleep(_) => "Zzz",
                            // CommandType::ToRest(_) => "",
                            // CommandType::UserSelection(_) => "",
                            CommandType::WorkOn(_) => "Working",
                            _ => "",
                        })
                        .into()
                    } else {
                        // *visibility = Visibility::Hidden;
                        String::new()
                    }
                }
            };
        }
    }
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
                // event_writer.send(log_event!(PawnBirthdayEvent(entity)));
            }
            pawn.decrease_lifetime(config().time.day_duration);

            if pawn.lifetime <= config().time.day_duration {
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
        pawn.decrease_lifetime(time_scale.scale_to_seconds(time.delta_secs()));

        if pawn.lifetime.is_zero() {
            event_writer.send(log_event!(PawnDeathEvent {
                entity,
                reason: PawnDeathReason::OldAge
            }));
            commands.entity(entity).remove::<DyingMarker>();
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn progress_pawn_death(
    mut commands: Commands,
    mut event_reader: EventReader<PawnDeathEvent>,
    mut pawn_query: Query<(&mut Pawn, &mut Restable, &mut Commandable)>,
    mut bed_query: Query<&mut Bed>,
    mut commandable_interrupt_writer: EventWriter<InternalCommandInterruptEvent>,
    mut commandable_release_resources_writer: EventWriter<ReleaseCommandResourcesEvent>,
    mut available_beds: ResMut<AvailableBeds>,
    mut pawn_state_change_event_writer: EventWriter<EntityStateChangeEvent<PawnState>>,
) {
    for PawnDeathEvent { entity, .. } in event_reader.read() {
        match pawn_query.get_mut(*entity) {
            Ok((mut pawn, mut restable, mut commandable)) => {
                pawn.change_state(
                    PawnState::Dead,
                    *entity,
                    &mut commands,
                    &mut pawn_state_change_event_writer,
                );

                restable.change_state(RestableState::Dead, *entity);

                commandable.clear_queue(
                    *entity,
                    &mut commands,
                    &mut commandable_interrupt_writer,
                    &mut commandable_release_resources_writer,
                );

                if let Some(bed_entity) = pawn.owned_bed {
                    let mut bed = bed_query.get_mut(bed_entity).unwrap();
                    bed.unclaim_by(&mut pawn, &mut available_beds);
                }
            }
            Err(err) => {
                warn!("Failed to get query result: {:?}", err);
                continue;
            }
        }
    }
}
