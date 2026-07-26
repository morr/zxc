use super::*;

pub fn insert_unsynced_noise_texture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let size = config().grid.size as usize;
    let grid_world_size = config().grid.size as f32 * config().tile.size;

    let height_texture = create_blank_texture(size as u32, size as u32);
    let height_texture_handle = images.add(height_texture);
    let height_mesh_handle = meshes.add(Rectangle::new(grid_world_size, grid_world_size));
    let height_material_handle = materials.add(ColorMaterial::from_color(Color::BLACK));

    let humidity_texture = create_blank_texture(size as u32, size as u32);
    let humidity_texture_handle = images.add(humidity_texture);
    let humidity_mesh_handle = meshes.add(Rectangle::new(grid_world_size, grid_world_size));
    let humidity_material_handle = materials.add(ColorMaterial::from_color(Color::BLACK));

    let props_texture = create_blank_texture(size as u32, size as u32);
    let props_texture_handle = images.add(props_texture);
    let props_mesh_handle = meshes.add(Rectangle::new(grid_world_size, grid_world_size));
    let props_material_handle = materials.add(ColorMaterial::from_color(Color::BLACK));

    commands.insert_resource(NoiseVisuals {
        height_noise: NoiseVisual {
            noise_type: NoiseType::Height,
            texture_handle: height_texture_handle,
            mesh_handle: height_mesh_handle,
            material_handle: height_material_handle,
            is_synced: false,
        },
        humidity_noise: NoiseVisual {
            noise_type: NoiseType::Humidity,
            texture_handle: humidity_texture_handle,
            mesh_handle: humidity_mesh_handle,
            material_handle: humidity_material_handle,
            is_synced: false,
        },
        props_noise: NoiseVisual {
            noise_type: NoiseType::Props,
            texture_handle: props_texture_handle,
            mesh_handle: props_mesh_handle,
            material_handle: props_material_handle,
            is_synced: false,
        },
    });
}

pub fn on_debug_noise_state_change(
    event: On<StateChangeEvent<DebugNoiseState>>,
    mut commands: Commands,
    mut noise_visuals: ResMut<NoiseVisuals>,
    // for sync_noise_texture
    tile_grid: Res<TileGrid>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result {
    // the previous state's mesh is despawned by its DespawnOnExit
    let noise_visual = match event.0 {
        DebugNoiseState::HeightNoise => &mut noise_visuals.height_noise,
        DebugNoiseState::HumidityNoise => &mut noise_visuals.humidity_noise,
        DebugNoiseState::PropsNoise => &mut noise_visuals.props_noise,
        DebugNoiseState::Hidden => return Ok(()),
    };

    if !noise_visual.is_synced {
        sync_noise_texture(noise_visual, &tile_grid, &mut images, &mut materials)?;
    }

    spawn_noise_mesh(&mut commands, noise_visual, event.0.clone());
    Ok(())
}

pub fn on_rebuild_map_complete(
    _event: On<RebuildMapCompleteEvent>,
    mut noise_visuals: ResMut<NoiseVisuals>,
    state: Res<State<DebugNoiseState>>,
    // for sync_noise_texture
    tile_grid: Res<TileGrid>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result {
    noise_visuals.height_noise.is_synced = false;
    noise_visuals.humidity_noise.is_synced = false;
    noise_visuals.props_noise.is_synced = false;

    let noise_visual = match state.get() {
        DebugNoiseState::HeightNoise => &mut noise_visuals.height_noise,
        DebugNoiseState::HumidityNoise => &mut noise_visuals.humidity_noise,
        DebugNoiseState::PropsNoise => &mut noise_visuals.props_noise,
        DebugNoiseState::Hidden => return Ok(()),
    };

    // handles are stable, so mutating the assets updates the visible mesh
    // in place — no despawn/respawn needed
    sync_noise_texture(noise_visual, &tile_grid, &mut images, &mut materials)?;
    Ok(())
}

pub fn sync_noise_texture(
    noise_visual: &mut NoiseVisual,
    tile_grid: &TileGrid,
    images: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Result {
    let noise_map = extract_tile_noise_map(tile_grid, &noise_visual.noise_type);
    let texture = render_noise_to_texture(&noise_map, &noise_visual.noise_type);
    let material = ColorMaterial::from(noise_visual.texture_handle.clone());

    *images
        .get_mut(&noise_visual.texture_handle)
        .ok_or("noise texture asset missing")? = texture;
    *materials
        .get_mut(&noise_visual.material_handle)
        .ok_or("noise material asset missing")? = material;

    noise_visual.is_synced = true;
    Ok(())
}

fn spawn_noise_mesh(commands: &mut Commands, noise_visual: &NoiseVisual, state: DebugNoiseState) {
    commands.spawn((
        DebugNoise,
        DespawnOnExit(state),
        Mesh2d(noise_visual.mesh_handle.clone()),
        MeshMaterial2d(noise_visual.material_handle.clone()),
        Transform::from_xyz(0.0, 0.0, TILE_Z_INDEX + 2.0),
    ));
}
