use super::*;

pub fn generate_map(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    // pn_config: Res<generator::perlin_noise::PerlinNoiseConfig>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let mut navmesh = arc_navmesh.write();
    let grid = generator::perlin_noise::generate();

    spawn_tiles(&mut commands, &assets, &mut navmesh, &grid);
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
