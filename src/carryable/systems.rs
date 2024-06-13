use bevy::sprite::MaterialMesh2dBundle;

use super::*;

pub fn spawn_on_event(
    mut event_reader: EventReader<SpawnCarryableEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets_collection: Res<AssetsCollection>,
    mut food: ResMut<FoodStock>,
    arc_navmesh: Res<ArcNavmesh>,
) {
    for event in event_reader.read() {
        let mesh = Mesh::from(Rectangle::new(
            config().tile.size / 4.0,
            config().tile.size / 4.0,
        ));
        let mesh_handle: Handle<Mesh> = meshes.add(mesh);

        let component = match event.kind {
            CarryableKind::Food => Carryable {
                kind: CarryableKind::Food,
                amount: event.amount,
            },
        };

        let carryable_id = commands
            .spawn((
                component,
                MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    material: assets_collection.food.clone(),
                    transform: Transform::from_translation(
                        event
                            .grid_tile
                            .grid_tile_center_to_world()
                            .extend(ITEM_Z_INDEX),
                    ),
                    ..default()
                },
            ))
            .id();

        // increment food stock
        match event.kind {
            CarryableKind::Food => {
                **food += event.amount;
            }
        };

        arc_navmesh.write().add_occupant::<Carryable>(
            carryable_id,
            event.grid_tile.x,
            event.grid_tile.y,
        );
    }
}

pub fn spawn_initial_items(mut event_writer: EventWriter<SpawnCarryableEvent>) {
    event_writer.send(SpawnCarryableEvent {
        kind: CarryableKind::Food,
        amount: config().starting_scene.food,
        grid_tile: IVec2 { x: -8, y: 0 },
    });
}
