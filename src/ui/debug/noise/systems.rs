use super::*;

pub fn insert_invalid_noise_texture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // let noise_map = extract_tile_noise_map(&tile_query);
    // let texture = render_noise_to_texture(&noise_map);

    let size = config().grid.size as usize;
    let grid_world_size = config().grid.size as f32 * config().tile.size;

    let texture = create_blank_texture(size as u32, size as u32);

    let texture_handle = images.add(texture);
    let mesh_handle = meshes.add(Rectangle::new(grid_world_size, grid_world_size));
    let material_handle = materials.add(ColorMaterial::from_color(Color::BLACK));

    commands.insert_resource(NoiseTexture {
        texture_handle,
        mesh_handle,
        material_handle,
        is_synced: false,
    });
}

#[allow(clippy::too_many_arguments)]
pub fn on_debug_noise_state_change(
    event: On<StateChangeEvent<DebugNoiseState>>,
    mut commands: Commands,
    query_mesh: Query<Entity, With<DebugNoise>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut noise_texture: ResMut<NoiseTexture>,
    // for sync_noise_texture
    tile_query: Query<&Tile>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let StateChangeEvent(ref state) = *event;
    match state {
        DebugNoiseState::Visible => {
            println!("DebugNoiseState::Hidden => DebugNoiseState::Visible");

            if !noise_texture.is_synced {
                sync_noise_texture(&mut noise_texture, &tile_query, &mut images, &mut materials);
            }

            // let handle = images.add(texture);
            // commands.insert_resource(NoiseTextureHandle(handle));

            spawn_noise_mesh(
                &mut commands,
                &noise_texture, // noise_texture.texture_handle.clone(),
                                // &mut meshes,
                                // &mut materials,
            );
        }
        DebugNoiseState::Hidden => {
            println!("DebugNoiseState::Visible => DebugNoiseState::Hidden");
            despawn_noise_texture(&mut commands, &query_mesh);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn on_rebuild_map_complete(
    _event: On<RebuildMapCompleteEvent>,
    mut commands: Commands,
    mut noise_texture: ResMut<NoiseTexture>,
    state: Res<State<DebugNoiseState>>,
    query_mesh: Query<Entity, With<DebugNoise>>,
    // for sync_noise_texture
    tile_query: Query<&Tile>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("on_rebuild_map texture");
    noise_texture.is_synced = false;

    if *state.get() == DebugNoiseState::Visible {
        despawn_noise_texture(&mut commands, &query_mesh);
        sync_noise_texture(&mut noise_texture, &tile_query, &mut images, &mut materials);
        spawn_noise_mesh(&mut commands, &noise_texture);
    }
}

fn despawn_noise_texture(commands: &mut Commands, query_mesh: &Query<Entity, With<DebugNoise>>) {
    if let Ok(entity) = query_mesh.single() {
        commands.entity(entity).despawn();
    }
}

pub fn sync_noise_texture(
    noise_texture: &mut ResMut<NoiseTexture>,
    tile_query: &Query<&Tile>,
    images: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let noise_map = extract_tile_noise_map(tile_query);
    let texture = render_noise_to_texture(&noise_map);
    let material = ColorMaterial::from(noise_texture.texture_handle.clone());

    *images.get_mut(&noise_texture.texture_handle).unwrap() = texture;
    *materials.get_mut(&noise_texture.material_handle).unwrap() = material;

    noise_texture.is_synced = true
}

// #[allow(clippy::too_many_arguments)]
// pub fn on_rebuild_map(
//     _event: On<RebuildMapEvent>,
//     mut images: ResMut<Assets<Image>>,
//     tile_query: Query<&Tile>,
//     noise_texture: Option<Res<NoiseTextureHandle>>,
//     current_state: Res<State<DebugNoiseState>>,
//     query_mesh: Query<Entity, With<DebugNoise>>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let noise_map = extract_tile_noise_map(&tile_query);
//     let texture = render_noise_to_texture(&noise_map);
//
//     // Update or create the texture resource
//     let texture_handle = if let Some(texture_res) = noise_texture {
//         // Update existing texture
//         *images.get_mut(&texture_res.0).unwrap() = texture;
//         texture_res.0.clone()
//     } else {
//         // Create new texture resource
//         let handle = images.add(texture);
//         commands.insert_resource(NoiseTextureHandle(handle.clone()));
//         handle
//     };
//
//     // If the noise visualization is currently visible, update the visualization too
//     if *current_state.get() == DebugNoiseState::Visible {
//         refresh_noise_visualization(
//             &mut commands,
//             texture_handle,
//             &mut meshes,
//             &mut materials,
//             &query_mesh,
//         );
//     }
// }
