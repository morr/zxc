use super::*;

// use crate::{
//     navigation::components::Navmesh, utils::GridTranslationHelper, TILE_SIZE, TILE_Z_INDEX,
// };

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugMovepathState {
    // MainMenu,
    #[default]
    Hidden,
    Visible,
}

pub struct DebugMovepathPlugin;
impl Plugin for DebugMovepathPlugin {
    fn build(&self, app: &mut App) {
        // app.add_event::<StateChangeEvent<DebugMovepathState>>()
        //     .init_state::<DebugMovepathState>()
        //     .add_systems(FixedUpdate, handle_state_changes);
    }
}

// fn handle_state_changes(
//     mut commands: Comands,
//     navmesh: Res<Navmesh>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     mut event_reader: EventReader<StateChangeEvent<DebugNavmeshState>>,
//     query_tiles_hovered: Query<Entity, With<DebugNavmeshTile>>,
// ) {
//     for event in event_reader.read() {
//         // println!("{:?}", event);
//
//         let mesh = Mesh::from(Rectangle::new(TILE_SIZE, TILE_SIZE));
//         let passable_material = ColorMaterial::from(Color::rgba(0.0, 0.0, 0.75, 0.5));
//         let impassable_material = ColorMaterial::from(Color::rgba(1.0, 0.0, 0.0, 0.75));
//
//         let mesh_handle = meshes.add(mesh);
//         let material_passable_handle = materials.add(passable_material);
//         let material_impassable_handle = materials.add(impassable_material);
//
//         match event.0 {
//             DebugNavmeshState::Visible => {
//                 navmesh.for_each_tile_mut(|navtile, tile_pos| {
//                     commands
//                         .spawn(MaterialMesh2dBundle {
//                             mesh: mesh_handle.clone().into(),
//                             material: if navtile.passable {
//                                 material_passable_handle.clone()
//                             } else {
//                                 material_impassable_handle.clone()
//                             },
//                             transform: Transform::from_translation(
//                                 tile_pos
//                                     .grid_tile_center_to_world()
//                                     .extend(TILE_Z_INDEX + 1.0),
//                             ),
//                             ..default()
//                         })
//                         .insert(DebugNavmeshTile);
//                 });
//             }
//             DebugNavmeshState::Hidden => {
//                 for entity in query_tiles_hovered.iter() {
//                     commands.entity(entity).despawn_recursive();
//                 }
//             }
//         }
//     }
// }
// m
