use bevy::sprite::MaterialMesh2dBundle;

use super::*;

pub fn spawn_item_on_event(
    mut event_reader: EventReader<SpawnItemEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets_collection: Res<AssetsCollection>,
    mut food: ResMut<FoodStock>
) {
    for event in event_reader.read() {
        let mesh = Mesh::from(Rectangle::new(
            config().tile.size / 4.0,
            config().tile.size / 4.0,
        ));
        let mesh_handle: Handle<Mesh> = meshes.add(mesh);

        let component = match event.item_type {
            CarryableKind::Food => Carryable {
                amount: event.amount,
                kind: CarryableKind::Food
            },
        };

        commands.spawn((
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
        ));

        // increment food stock
        match event.item_type {
            CarryableKind::Food => {
                **food += event.amount;
            }
        };
    }
}
