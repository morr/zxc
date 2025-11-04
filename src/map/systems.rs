use super::*;
use rand::Rng;

pub fn generate_map(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    tree_assets: Res<TreeAssets>,
    // pn_config: Res<generator::perlin_noise::PerlinNoiseConfig>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let mut navmesh = arc_navmesh.write();
    let grid = generator::perlin_noise::generate();

    spawn_tiles(&mut commands, &texture_assets, &mut navmesh, &grid);
    spawn_trees(&mut commands, &tree_assets, &mut navmesh, &grid);
}

fn spawn_tiles(
    commands: &mut Commands,
    assets: &Res<TextureAssets>,
    navmesh: &mut Navmesh,
    grid: &[Vec<Tile>],
) {
    for row in grid.iter() {
        for tile in row.iter() {
            let id = commands
                .spawn((
                    Sprite {
                        image: tile.texture(assets),
                        custom_size: Some(Vec2::new(config().tile.size, config().tile.size)),
                        ..default()
                    },
                    Transform::from_xyz(
                        grid_tile_edge_to_world(tile.grid_tile.x) + config().tile.size / 2.,
                        grid_tile_edge_to_world(tile.grid_tile.y) + config().tile.size / 2.,
                        TILE_Z_INDEX,
                    ),
                ))
                .insert(*tile)
                .id();

            navmesh.add_occupant::<Tile>(&id, tile.grid_tile.x, tile.grid_tile.y);
            // no need to inform about occupation change for spawned empty map tiles
            // occupation_change_event_writer.write(log_message!(OccupationChangeEvent::new(grid_tile)));
        }
    }
}

fn spawn_trees(
    commands: &mut Commands,
    assets: &Res<TreeAssets>,
    _navmesh: &mut Navmesh,
    grid: &[Vec<Tile>],
) {
    let mut rng = rand::rng();

    let tree_variants: [(Handle<Image>, f32); 14] = [
        (assets.tree_1x3_1.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_2.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_3.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_4.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_5.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_6.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_7.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_8.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_2x3_1.clone(), TreeAssets::ASPECT_RATIO_2X3),
        (assets.tree_2x3_2.clone(), TreeAssets::ASPECT_RATIO_2X3),
        (assets.tree_2x3_3.clone(), TreeAssets::ASPECT_RATIO_2X3),
        (assets.tree_2x3_4.clone(), TreeAssets::ASPECT_RATIO_2X3),
        (assets.tree_3x4_1.clone(), TreeAssets::ASPECT_RATIO_3X4),
        (assets.tree_3x4_2.clone(), TreeAssets::ASPECT_RATIO_3X4),
    ];

    for row in grid.iter().rev() {
        for tile in row.iter().rev() {
            // if tile.height_noise >= 0.6 && tile.humidity_noise >= 0.6 && tile.props_noise >= 0.6 {
            if tile.height_noise >= 0.6 && tile.props_noise >= 0.5 {
                let (tree_image, aspect_ratio) = tree_variants[rng.random_range(0..14)].clone();
                let tile_item = TileItem {
                    grid_tile: tile.grid_tile,
                    width: 1,
                    height: 1,
                    aspect_ratio,
                    z_index: PROP_Z_INDEX,
                    movement_cost: 1.0,
                };
                let random_angle: f32 = rng.random_range(0.0..360.0);

                commands.spawn((
                    Sprite {
                        image: tree_image,
                        custom_size: Some(tile_item.sprite_size()),
                        ..default()
                    },
                    tile_item.sprite_transform(Some(Vec2 {
                        x: random_angle.cos() * 0.35,
                        y: random_angle.sin().abs() * 0.5,
                    })),
                ));
            }
        }
    }
}

pub fn track_hover(
    mut commands: Commands,
    mut event_reader: MessageReader<HoverMessage>,
    arc_navmesh: Res<ArcNavmesh>,
    q_hover_markers: Query<(Entity, &Tile), With<HoverMarker>>,
    // q_tiles: Query<(Entity, &Tile)>,
) {
    for event in event_reader.read() {
        // remove hover markers from other tiles
        for (id, _tile) in q_hover_markers.iter() {
            commands.entity(id).remove::<HoverMarker>();
            // .remove::<ShowAabbGizmo>();
        }

        let navmesh = arc_navmesh.read();

        for id in navmesh.get_occupants::<Tile>(event.0.x, event.0.y) {
            commands.entity(*id).insert(HoverMarker);
            // .insert(ShowAabbGizmo {
            //     color: Some(*Color::WHITE.clone().set_a(0.25)),
            // });
        }
        // for (entity, tile) in q_tiles.iter() {
        //     if tile.0 == event.0 {
        //         commands.entity(entity).insert(HoverMarker);
        //         // .insert(ShowAabbGizmo {
        //         //     color: Some(*Color::WHITE.clone().set_a(0.25)),
        //         // });
        //         break;
        //     }
        // }
    }
}

pub fn on_rebuild_map(
    event: On<RebuildMapEvent>,
    mut commands: Commands,
    assets: Res<TextureAssets>,
    arc_navmesh: ResMut<ArcNavmesh>,
    tiles_query: Query<(Entity, &Tile)>,
) {
    let RebuildMapEvent { generator_kind } = *event;
    let mut navmesh = arc_navmesh.write();

    for (entity, tile) in tiles_query.iter() {
        navmesh.remove_occupant::<Tile>(&entity, tile.grid_tile.x, tile.grid_tile.y);
        commands.entity(entity).despawn();
    }

    let grid = match generator_kind {
        GeneratorKind::PerlinNoise => generator::perlin_noise::generate(),
    };

    spawn_tiles(&mut commands, &assets, &mut navmesh, &grid);
    commands.trigger(log_event!(RebuildMapCompleteEvent));
}
