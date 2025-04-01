use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugNoiseState {
    // MainMenu,
    #[default]
    Hidden,
    Visible,
}

#[derive(Component)]
pub struct DebugNoiseTile;

pub struct DebugNoisePlugin;
impl Plugin for DebugNoisePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_state(if config().debug.is_noise {
                DebugNoiseState::Visible
            } else {
                DebugNoiseState::Hidden
            })
            .add_event::<StateChangeEvent<DebugNoiseState>>()
            .add_systems(
                FixedUpdate,
                handle_state_changes.run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                OnExit(AppState::Loading),
                initialize_noise_debug_tiles.run_if(in_state(AppState::Playing)),
            );
    }
}


fn initialize_noise_debug_tiles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetsCollection>,
    noise_state: Res<State<DebugNoiseState>>,
    query_tiles: Query<Entity, With<DebugNoiseTile>>,
) {
    update_noise_tiles_visibility(
        &mut commands,
        &mut meshes,
        &assets,
        &query_tiles,
        noise_state.get(),
    );
}

fn handle_state_changes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    assets: Res<AssetsCollection>,
    mut event_reader: EventReader<StateChangeEvent<DebugNoiseState>>,
    query_tiles: Query<Entity, With<DebugNoiseTile>>,
) {
    for event in event_reader.read() {
        update_noise_tiles_visibility(&mut commands, &mut meshes, &assets, &query_tiles, &event.0);
    }
}

fn update_noise_tiles_visibility(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    assets: &Res<AssetsCollection>,
    query_tiles: &Query<Entity, With<DebugNoiseTile>>,
    state: &DebugNoiseState, // Changed to reference
) {
    match state {
        DebugNoiseState::Visible => {
            // let mesh = Mesh::from(Rectangle::new(
            //     config().tile.size * config().grid.size as f32,
            //     config().tile.size * config().grid.size as f32,
            // ));
            // let mesh_handle = meshes.add(mesh);
            //
            // commands
            //     .spawn((
            //         Mesh2d(mesh_handle.clone()),
            //         MeshMaterial2d(assets.navmesh_passable.clone()),
            //         Transform::from_xyz(0.0, 0.0, TILE_Z_INDEX + 1.0),
            //     ))
            //     .insert(DebugNoiseTile);
        }
        DebugNoiseState::Hidden => {
            for entity in query_tiles.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
