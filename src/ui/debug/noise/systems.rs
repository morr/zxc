use super::*;

pub fn initialize_noise_texture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    tile_query: Query<&Tile>,
) {
    let noise_map = extract_tile_noise_map(&tile_query);
    let texture = render_noise_to_texture(&noise_map);
    let handle = images.add(texture);
    commands.insert_resource(NoiseTextureHandle(handle));
}

pub fn on_debug_noise_state_change(
    event: On<StateChangeEvent<DebugNoiseState>>,
    mut commands: Commands,
    query_mesh: Query<Entity, With<DebugNoise>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    noise_texture: Res<NoiseTextureHandle>,
) {
    let StateChangeEvent(ref state) = *event;
    match state {
        DebugNoiseState::Visible => {
            println!("DebugNoiseState::Hidden => DebugNoiseState::Visible");
            refresh_noise_visualization(
                &mut commands,
                noise_texture.0.clone(),
                &mut meshes,
                &mut materials,
                &query_mesh,
            );
        }
        DebugNoiseState::Hidden => {
            println!("DebugNoiseState::Visible => DebugNoiseState::Hidden");
            commands.entity(query_mesh.single().unwrap()).despawn();
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn on_rebuild_map(
    _event: On<RebuildMapEvent>,
    mut images: ResMut<Assets<Image>>,
    tile_query: Query<&Tile>,
    noise_texture: Option<Res<NoiseTextureHandle>>,
    current_state: Res<State<DebugNoiseState>>,
    query_mesh: Query<Entity, With<DebugNoise>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let noise_map = extract_tile_noise_map(&tile_query);
    let texture = render_noise_to_texture(&noise_map);

    // Update or create the texture resource
    let texture_handle = if let Some(texture_res) = noise_texture {
        // Update existing texture
        *images.get_mut(&texture_res.0).unwrap() = texture;
        texture_res.0.clone()
    } else {
        // Create new texture resource
        let handle = images.add(texture);
        commands.insert_resource(NoiseTextureHandle(handle.clone()));
        handle
    };

    // If the noise visualization is currently visible, update the visualization too
    if *current_state.get() == DebugNoiseState::Visible {
        refresh_noise_visualization(
            &mut commands,
            texture_handle,
            &mut meshes,
            &mut materials,
            &query_mesh,
        );
    }
}

fn refresh_noise_visualization(
    commands: &mut Commands,
    texture_handle: Handle<Image>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    query_mesh: &Query<Entity, With<DebugNoise>>,
) {
    // First, remove the old visualization if it exists
    for entity in query_mesh.iter() {
        commands.entity(entity).despawn();
    }

    // Create a new visualization
    let grid_world_size = config().grid.size as f32 * config().tile.size;
    let mesh = meshes.add(Rectangle::new(grid_world_size, grid_world_size));
    let material = materials.add(ColorMaterial::from(texture_handle));

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, TILE_Z_INDEX + 2.0),
        DebugNoise { is_synced: true },
    ));
}
