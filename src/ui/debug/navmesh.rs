use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugNavmeshState {
    #[default]
    Hidden,
    Visible,
}

#[derive(Component)]
pub struct DebugNavmeshTile;

pub struct DebugNavmeshPlugin;
impl Plugin for DebugNavmeshPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(DebugNavmeshState::Hidden)
            .add_observer(on_debug_navmesh_state_change);

        if config().debug.is_navmesh {
            app.add_systems(
                OnExit(AppState::Loading),
                (|mut next_state: ResMut<NextState<DebugNavmeshState>>, mut commands: Commands| {
                    next_state.set(DebugNavmeshState::Visible);
                    commands.trigger(log_event!(StateChangeEvent(DebugNavmeshState::Visible)));
                })
                .after(generate_map),
            );
        }
    }
}

fn on_debug_navmesh_state_change(
    event: On<StateChangeEvent<DebugNavmeshState>>,
    mut commands: Commands,
    arc_navmesh: Res<ArcNavmesh>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetsCollection>,
    query_tiles: Query<Entity, With<DebugNavmeshTile>>,
) {
    let StateChangeEvent(ref state) = *event;
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
                commands.entity(entity).despawn();
            }
        }
    }
}
