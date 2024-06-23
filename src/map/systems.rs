use super::*;

pub fn spawn_map(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    arc_navmesh: ResMut<ArcNavmesh>,
    // mut occupation_change_event_writer: EventWriter<OccupationChangeEvent>,
) {
    // println!("spawn map");
    let mut navmesh = arc_navmesh.write();

    for x in -config().grid.half_size..config().grid.half_size {
        for y in -config().grid.half_size..config().grid.half_size {
            let grid_tile = IVec2::new(x, y);

            let id = commands
                .spawn(SpriteBundle {
                    texture: assets.grass.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(config().tile.size, config().tile.size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        grid_tile_edge_to_world(x) + config().tile.size / 2.,
                        grid_tile_edge_to_world(y) + config().tile.size / 2.,
                        TILE_Z_INDEX,
                    ),
                    ..default()
                })
                .insert(Tile(grid_tile))
                .id();

            navmesh.add_occupant::<Tile>(&id, grid_tile.x, grid_tile.y);
            // no need to inform about occupation change for spawned empty map tiles
            // occupation_change_event_writer.send(log_event!(OccupationChangeEvent::new(grid_tile)));
        }
    }
}

pub fn track_hover(
    mut commands: Commands,
    mut event_reader: EventReader<HoverEvent>,
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
