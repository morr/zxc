use bevy::sprite::MaterialMesh2dBundle;

use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugNavmeshState {
    // MainMenu,
    #[default]
    Hidden,
    Visible,
}

#[derive(Component)]
pub struct DebugNavmeshTile;

pub struct DebugNavmeshPlugin;
impl Plugin for DebugNavmeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StateChangeEvent<DebugNavmeshState>>()
            .init_state::<DebugNavmeshState>()
            .add_systems(FixedUpdate, handle_state_changes.run_if(in_state(WorldState::Playing)));
    }
}

fn handle_state_changes(
    mut commands: Commands,
    arc_navmesh: Res<ArcNavmesh>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetsCollection>,
    mut event_reader: EventReader<StateChangeEvent<DebugNavmeshState>>,
    query_tiles_hovered: Query<Entity, With<DebugNavmeshTile>>,
) {
    for event in event_reader.read() {
        // println!("{:?}", event);

        let mesh = Mesh::from(Rectangle::new(CONFIG.tile.size, CONFIG.tile.size));
        let mesh_handle = meshes.add(mesh);

        match event.0 {
            DebugNavmeshState::Visible => {
                arc_navmesh.read().navtiles.for_each_tile_mut(|navtile, tile_pos| {
                    commands
                        .spawn(MaterialMesh2dBundle {
                            mesh: mesh_handle.clone().into(),
                            material: if navtile.is_passable() {
                                assets.navmesh_passable.clone()
                            } else {
                                assets.navmesh_impassable.clone()
                            },
                            transform: Transform::from_translation(
                                tile_pos
                                    .grid_tile_center_to_world()
                                    .extend(TILE_Z_INDEX + 1.0),
                            ),
                            ..default()
                        })
                        .insert(DebugNavmeshTile);
                });
            }
            DebugNavmeshState::Hidden => {
                for entity in query_tiles_hovered.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}
