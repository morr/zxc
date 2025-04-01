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
        app.insert_state(if config().debug.is_navmesh {
            DebugNavmeshState::Visible
        } else {
            DebugNavmeshState::Hidden
        })
        .add_event::<StateChangeEvent<DebugNavmeshState>>()
        .add_systems(
            FixedUpdate,
            handle_state_changes.run_if(in_state(AppState::Playing)),
        );
    }
}

fn handle_state_changes(
    mut commands: Commands,
    arc_navmesh: Res<ArcNavmesh>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetsCollection>,
    mut event_reader: EventReader<StateChangeEvent<DebugNavmeshState>>,
    query_tiles: Query<Entity, With<DebugNavmeshTile>>,
) {
    for StateChangeEvent(state) in event_reader.read() {
        match state {
            DebugNavmeshState::Visible => {
                let mesh = Mesh::from(Rectangle::new(config().tile.size, config().tile.size));
                let mesh_handle = meshes.add(mesh);

                arc_navmesh
                    .read()
                    .navtiles
                    .for_each_tile_mut(|navtile, tile_pos| {
                        commands
                            .spawn((
                                Mesh2d(mesh_handle.clone()),
                                MeshMaterial2d(if navtile.is_passable() {
                                    assets.navmesh_passable.clone()
                                } else {
                                    assets.navmesh_impassable.clone()
                                }),
                                Transform::from_translation(
                                    tile_pos
                                        .grid_tile_center_to_world()
                                        .extend(TILE_Z_INDEX + 1.0),
                                ),
                            ))
                            .insert(DebugNavmeshTile);
                    });
            }
            DebugNavmeshState::Hidden => {
                for entity in query_tiles.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}
