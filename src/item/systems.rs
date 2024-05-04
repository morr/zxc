use super::*;

pub fn spawn_item_on_event(mut food: ResMut<Food>, mut event_reader: EventReader<SpawnItemEvent>) {
    for event in event_reader.read() {
        match event.item_type {
            ItemType::Food => {
                food.0 += event.amount;
            }
        };
    }
}

// pub fn spawn_item_on_event(
//     mut event_reader: EventReader<SpawnItemEvent>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     assets_collection: Res<AssetsCollection>,
// ) {
//     for event in event_reader.read() {
//         let mesh = Mesh::from(Rectangle::new(
//             CONFIG.tile.size / 4.0,
//             CONFIG.tile.size / 4.0,
//         ));
//         let mesh_handle: Handle<Mesh> = meshes.add(mesh);
//
//         let component = match event.item_type {
//             ItemType::Food => FoodItem {
//                 amount: event.amount,
//             },
//         };
//
//         commands.spawn((
//             component,
//             MaterialMesh2dBundle {
//                 mesh: mesh_handle.clone().into(),
//                 material: assets_collection.food.clone(),
//                 transform: Transform::from_translation(
//                     event
//                         .grid_tile
//                         .grid_tile_center_to_world()
//                         .extend(ITEM_Z_INDEX),
//                 ),
//                 ..default()
//             },
//         ));
//     }
// }
