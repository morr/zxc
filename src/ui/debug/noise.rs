use super::*;
use ::noise::Perlin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugNoiseState {
    // MainMenu,
    #[default]
    Hidden,
    Visible,
}

#[derive(Component)]
pub struct DebugNoiseTile;

#[derive(Resource)]
pub struct NoiseSettings {
    pub seed: u32,
    pub frequency: f64,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            // seed: rand::random(),
            seed: 1655470700,
            frequency: 0.01,
            octaves: 4,
            lacunarity: 2.0,
            persistence: 0.5,
            offset_x: 0,
            offset_y: 0,
        }
    }
}

#[derive(Resource, Default)]
pub struct NoiseData(Option<Vec<f32>>);

pub struct DebugNoisePlugin;
impl Plugin for DebugNoisePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoiseSettings>()
            .init_resource::<NoiseData>()
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
            .add_systems(Startup, initialize_noise_data)
            .add_systems(
                OnExit(AppState::Loading),
                initialize_noise_debug_tiles.run_if(in_state(AppState::Playing)),
            );
    }
}

fn initialize_noise_data(noise_settings: Res<NoiseSettings>, mut noise_data: ResMut<NoiseData>) {
    let perlin = Perlin::new(noise_settings.seed);

    let width = config().grid.size as usize;
    let height = config().grid.size as usize;

    let mut data = vec![0.0f32; width * height];

    // Generate noise values for each pixel in the texture
    for y in 0..height {
        for x in 0..width {
            let offseted_x = x as i32 + noise_settings.offset_x;
            let offseted_y = y as i32 + noise_settings.offset_y;

            // scale to the grid range and apply frequency
            let nx = offseted_x as f64 * noise_settings.frequency;
            let ny = offseted_y as f64 * noise_settings.frequency;

            let mut noise_value = 0.0;
            let mut amplitude = 1.0;
            let mut frequency = 1.0;

            // Generate octaves of noise
            for _ in 0..noise_settings.octaves {
                noise_value +=
                    ::noise::NoiseFn::get(&perlin, [nx * frequency, ny * frequency, 0.0])
                        * amplitude;
                amplitude *= noise_settings.persistence;
                frequency *= noise_settings.lacunarity;
            }

            // normalize to 0.0 - 1.0
            let normalized_value = ((noise_value + 1.0) / 2.0) as f32;
            let index = y * width + x;

            data[index] = normalized_value;
        }
    }

    noise_data.0 = Some(data);
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
