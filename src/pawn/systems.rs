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
) {
    let mut rng = rand::rng();
    let radius = config().tile.size * i32::max(BASE_WIDTH, BASE_HEIGHT) as f32;

    // let warehouse_transform = warehouse_query.single();
    let maybe_farm_transform = farm_query.iter().next();

    let mut navmesh = arc_navmesh.write();
    for i in 0..config().starting_scene.pawns {
        let random_angle: f32 = rng.random_range(0.0..360.0);

        let position = if i < 4
            && let Some(farm_transform) = maybe_farm_transform
        {
            Vec3::new(
                farm_transform.translation.x
                    + random_angle.cos() * 5.0 * FARM_TILE_SIZE as f32 * config().tile.size,
                farm_transform.translation.y
                    + random_angle.sin() * 5.0 * FARM_TILE_SIZE as f32 * config().tile.size,
                PAWN_Z_INDEX,
            )
        } else {
            Vec3::new(
                random_angle.cos() * radius,
                random_angle.sin() * radius,
                // warehouse_transform.translation.x + random_angle.cos() * radius,
                // warehouse_transform.translation.y + random_angle.sin() * radius,
                PAWN_Z_INDEX,
            )
        };
        let pawn_id = commands
            .spawn((
                (
                    Pawn::default(),
                    Mesh2d(meshes_collection.pawn.clone()),
                    MeshMaterial2d(assets_collection.pawn_idle.clone()),
                    Transform::from_translation(position),
                ),
                children![(
                    Text2d::new(""),
                    TextFont {
                        font: font_assets.fira.clone().into(),
                        font_size: FontSize::Px(13.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, 21.0, PAWN_Z_INDEX),
                    PawnStateText,
                )],
            ))
            // .insert(ShowAabbGizmo {
            //     color: Some(Color::srgba(1.0, 1.0, 1.0, 0.25)),
            // })
            .id();

        let grid_tile = position.truncate().world_pos_to_grid();
        navmesh.add_occupant::<Pawn>(&pawn_id, grid_tile.x, grid_tile.y);
        commands.trigger(log_event!(OccupationChangeEvent::new(grid_tile)));
    }
}

pub fn on_pawn_entity_state_change(
    event: On<EntityStateChangeEvent<PawnState>>,
    children_query: Query<&Children>,
    // mut state_text_query: Query<(&mut Text, &mut Visibility), With<PawnStateText>>,
    // mut state_text_query: Query<&mut Text, With<PawnStateText>>,
    mut text_writer: Text2dWriter,
    commandable_query: Query<&Commandable>,
) {
    let EntityStateChangeEvent(pawn_entity, ref state) = *event;
    for text_entity in children_query.iter_descendants(pawn_entity) {
        // let (mut text, mut visibility) = state_text_query.get_mut(text_entity).unwrap();
        // let mut text = state_text_query.get_mut(text_entity).unwrap();

        // *visibility = Visibility::Visible;

        *text_writer.text(text_entity, 0) = match state {
            PawnState::Idle => "Idle".into(),
            PawnState::Dead => "DEAD".into(),
            PawnState::ExecutingCommand => {
                let commandable = commandable_query
                    .get(pawn_entity)
                    .expect("on_pawn_entity_state_change: Commandable query failed");
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

pub fn on_new_day_age_pawns(
    event: On<NewDayEvent>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Pawn), Without<pawn_state::PawnStateDeadTag>>,
) {
    for (entity, mut pawn) in query.iter_mut() {
        if pawn.is_birthday(event.0) {
            pawn.age += 1;
        }
        pawn.decrease_lifetime(config().time.day_duration);

        if pawn.lifetime <= config().time.day_duration {
            commands.entity(entity).insert(DyingMarker);
        }
    }
}

pub fn progress_pawn_dying(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Pawn), With<DyingMarker>>,
) {
    for (entity, mut pawn) in query.iter_mut() {
        pawn.decrease_lifetime(time.delta_secs());

        if pawn.lifetime.is_zero() {
            commands.trigger(log_event!(PawnDeathEvent {
                entity,
                reason: PawnDeathReason::OldAge
            }));
            commands.entity(entity).remove::<DyingMarker>();
        }
    }
}

pub fn on_pawn_death(
    event: On<PawnDeathEvent>,
    mut commands: Commands,
    mut pawn_query: Query<(
        &mut Pawn,
        &mut Restable,
        &mut Commandable,
        Option<&OwnedBed>,
    )>,
) {
    let PawnDeathEvent { entity, .. } = *event;
    match pawn_query.get_mut(entity) {
        Ok((mut pawn, mut restable, mut commandable, owned_bed)) => {
            pawn.change_state(PawnState::Dead, entity, &mut commands);

            restable.change_state(RestableState::Dead, entity);
            commandable.clear_queue(entity, &mut commands);

            // dead pawns are not despawned, so release the bed explicitly
            if let Some(bed_entity) = owned_bed.and_then(OwnedBed::bed) {
                commands.entity(bed_entity).remove::<BedOwnedBy>();
            }
        }
        Err(err) => {
            warn!("Failed to get query result: {:?}", err);
            // return;
        }
    }
}
