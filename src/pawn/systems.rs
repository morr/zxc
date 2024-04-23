use bevy::sprite::MaterialMesh2dBundle;
use rand::Rng;
use rand_distr::{Distribution, UnitCircle};

use self::structure::{Warehouse, BASE_HEIGHT, BASE_WIDTH};

use super::*;

pub fn spawn_pawns(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets_collection: Res<AssetsCollection>,
    font_assets: Res<FontAssets>,
    query: Query<&Transform, With<Warehouse>>,
) {
    // println!("Spawning pawns");

    let mesh = Mesh::from(Circle::new(TILE_SIZE / 2.0 * 0.75));
    // let material = ColorMaterial::from(Color::hex("E178C5").unwrap());
    let mesh_handle: Handle<Mesh> = meshes.add(mesh);
    // let material_handle = materials.add(material);

    let mut rng = rand::thread_rng();
    let radius = TILE_SIZE * i32::max(BASE_WIDTH, BASE_HEIGHT) as f32;

    let transform = query.single();

    for _i in 0..STARTING_PAWNS {
        let random_angle: f32 = rng.gen_range(0.0..360.0);

        let position = Vec3::new(
            transform.translation.x + random_angle.cos() * radius,
            transform.translation.y + random_angle.sin() * radius,
            PAWN_Z_INDEX,
        );
        let pawn = Pawn::default();
        let pawn_state_string = format!("{:?}", pawn.state);

        commands
            .spawn((
                pawn,
                Name::new("Pawn"),
                // state: PawnState::Idle,
                MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    material: assets_collection.pawn_idle.clone(),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                Movement::new(PAWN_SPEED),
                PawnIdle,
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
    mut event_reader: EventReader<EntityStateChangeEvent<MovementState>>,
    mut query: Query<&mut Handle<ColorMaterial>>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);

        if let Ok(mut material_handle) = query.get_mut(event.0) {
            *material_handle = match event.1 {
                MovementState::Idle => assets_collection.pawn_idle.clone(),
                MovementState::Moving => assets_collection.pawn_moving.clone(),
                MovementState::Pathfinding(_end_tile) => assets_collection.pawn_pathfinding.clone(),
                MovementState::PathfindingError => assets_collection.pawn_pathfinding_error.clone(),
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
            &Transform,
            &mut Movement,
            Option<&mut PathfindingTask>,
        ),
        With<PawnIdle>,
    >,
    // time_scale: Res<TimeScale>,
    // mut pathfind_event_writer: EventWriter<PathfindRequestEvent>,
    mut movement_state_event_writer: EventWriter<EntityStateChangeEvent<MovementState>>,
) {
    let mut rng = rand::thread_rng();

    for (entity, transform, mut movement, mut maybe_pathfinding_task) in &mut query {
        if movement.state != MovementState::Idle {
            continue;
        }

        let world_pos = transform.translation.truncate();

        // movement.to_pathfinding(
        //     entity,
        //     world_pos.world_pos_to_grid(),
        //     (world_pos + move_vector * tiles_to_move).world_pos_to_grid(),
        //     &mut commands,
        //     &mut pathfind_event_writer,
        //     &mut movement_state_event_writer,
        // );
        movement.to_pathfinding_async(
            entity,
            world_pos.world_pos_to_grid(),
            find_valid_end_tile(world_pos, &*arc_navmesh.read(), &mut rng, 0),
            &arc_navmesh,
            &queue_counter,
            maybe_pathfinding_task.as_deref_mut(),
            &mut commands,
            &mut movement_state_event_writer,
        );
    }
}

const MAX_RECURSION_DEPTH: usize = 10;

fn find_valid_end_tile(
    start_pos: Vec2,
    navmesh: &Navmesh,
    rng: &mut impl Rng,
    recursion_depth: usize,
) -> IVec2 {
    let move_vector: Vec2 = UnitCircle.sample(rng).into();
    let tiles_to_move = rng.gen_range(3.0..12.0) * TILE_SIZE;
    let end_tile = (start_pos + move_vector * tiles_to_move).world_pos_to_grid();

    if recursion_depth >= MAX_RECURSION_DEPTH {
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
