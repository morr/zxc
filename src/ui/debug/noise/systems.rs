use super::*;

pub fn initialize_noise_texture(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // let noise_map = extract_tile_noise_map(&tile_query);
    // let texture = render_noise_to_texture(&noise_map);

    let size = config().grid.size as usize;
    let texture = create_blank_texture(size as u32, size as u32);
    let handle = images.add(texture);

    commands.insert_resource(NoiseTexture {
        handle,
        is_invalid: true,
    });
}

pub fn on_debug_noise_state_change(
    event: On<StateChangeEvent<DebugNoiseState>>,
    mut commands: Commands,
    query_mesh: Query<Entity, With<DebugNoise>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    // mut noise_texture: ResMut<NoiseTexture>,
) {
    let StateChangeEvent(ref state) = *event;
    match state {
        DebugNoiseState::Visible => {
            println!("DebugNoiseState::Hidden => DebugNoiseState::Visible");

            // let handle = images.add(texture);
            // commands.insert_resource(NoiseTextureHandle(handle));

            // spawn_noise_mesh_and_material(
            //     &mut commands,
            //     noise_texture.handle.clone(),
            //     &mut meshes,
            //     &mut materials,
            //     &query_mesh,
            // );
        }
        DebugNoiseState::Hidden => {
            println!("DebugNoiseState::Visible => DebugNoiseState::Hidden");
            commands.entity(query_mesh.single().unwrap()).despawn();
        }
    }
}

pub fn on_rebuild_map(
    _event: On<RebuildMapEvent>,
    mut commands: Commands,
    mut noise_texture: ResMut<NoiseTexture>,
    state: Res<State<DebugNoiseState>>,
    query_mesh: &Query<Entity, With<DebugNoise>>,
) {
    noise_texture.is_invalid = true;

    if *state.get() == DebugNoiseState::Visible {
        despawn_noise_texture(&mut commands, query_mesh);
    }
}

fn despawn_noise_texture(commands: &mut Commands, query_mesh: &Query<Entity, With<DebugNoise>>) {
    if let Ok(entity) = query_mesh.single() {
        commands.entity(entity).despawn();
    }
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
