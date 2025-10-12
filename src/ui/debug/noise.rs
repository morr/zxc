use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_platform::collections::HashMap;

use super::*;
use crate::map::RebuildMapEvent;

pub struct DebugNoisePlugin;
impl Plugin for DebugNoisePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(DebugNoiseState::Hidden)
            .add_message::<StateChangeMessage<DebugNoiseState>>()
            .add_systems(
                OnExit(AppState::Loading),
                initialize_noise_texture.after(generate_map),
            )
            .add_observer(on_rebuild_map)
            .add_systems(
                FixedUpdate,
                toggle_noise_visibility.run_if(in_state(AppState::Playing)),
            );

        if config().debug.is_noise {
            app.add_systems(
                OnExit(AppState::Loading),
                (|mut next_state: ResMut<NextState<DebugNoiseState>>,
                  mut debug_noise_state_change_event_writer: MessageWriter<
                    StateChangeMessage<DebugNoiseState>,
                >| {
                    next_state.set(DebugNoiseState::Visible);
                    debug_noise_state_change_event_writer
                        .write(log_message!(StateChangeMessage(DebugNoiseState::Visible)));
                })
                .after(generate_map)
                .after(initialize_noise_texture),
            );
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugNoiseState {
    #[default]
    Hidden,
    Visible,
}

#[derive(Component)]
pub struct DebugNoise;

#[derive(Resource)]
pub struct NoiseTextureHandle(pub Handle<Image>);

fn extract_tile_noise_map(tile_query: &Query<&Tile>) -> HashMap<(usize, usize), f32> {
    let size = config().grid.size as usize;
    let mut noise_map = HashMap::new();

    for tile in tile_query.iter() {
        let grid_pos = tile.grid_tile;
        // Convert from grid coordinates to noise texture coordinates
        let x = grid_tile_to_navmesh_index(grid_pos.x);
        let y = grid_tile_to_navmesh_index(grid_pos.y);

        if x < size && y < size {
            noise_map.insert((x, y), tile.noise_value);
        }
    }

    noise_map
}

fn render_noise_to_texture(noise_map: &HashMap<(usize, usize), f32>) -> Image {
    let size = config().grid.size as usize;
    let mut texture = create_blank_texture(size as u32, size as u32);
    let data = texture
        .data
        .as_mut()
        .expect("Texture data should be initialized");

    // Fill the texture using the noise values from tiles
    for y in 0..size {
        for x in 0..size {
            let noise_value = *noise_map.get(&(x, y)).unwrap_or(&0.0);

            // pixel index (y * width + x) * 4 for RGBA format
            // let texture_index = (y * size + x) * 4;
            // flip Y coordinate
            let texture_index = ((size - 1 - y) * size + x) * 4;
            // Convert to 0-255 for RGBA
            let rgb_value = (noise_value * 255.0) as u8;

            // Set RGBA values (grayscale with full opacity)
            data[texture_index] = rgb_value; // R
            data[texture_index + 1] = rgb_value; // G
            data[texture_index + 2] = rgb_value; // B
            data[texture_index + 3] = 255; // A (full opacity)
        }
    }

    texture
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
        DebugNoise,
    ));
}

fn initialize_noise_texture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    tile_query: Query<&Tile>,
) {
    // Get noise values from tiles
    let noise_map = extract_tile_noise_map(&tile_query);

    // Create texture
    let texture = render_noise_to_texture(&noise_map);

    // Add texture to assets and store handle in resource
    let handle = images.add(texture);
    commands.insert_resource(NoiseTextureHandle(handle));
}

#[allow(clippy::too_many_arguments)]
fn on_rebuild_map(
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
    // Get noise values from tiles
    let noise_map = extract_tile_noise_map(&tile_query);

    // Create texture
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

fn toggle_noise_visibility(
    mut commands: Commands,
    mut event_reader: MessageReader<StateChangeMessage<DebugNoiseState>>,
    query_mesh: Query<Entity, With<DebugNoise>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    noise_texture: Res<NoiseTextureHandle>,
) {
    for StateChangeMessage(state) in event_reader.read() {
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
}

fn create_blank_texture(width: u32, height: u32) -> Image {
    // calculate total pixels for rgba format (4 bytes per pixel)
    let pixel_count = width * height;
    let texture_data = vec![0u8; (pixel_count * 4) as usize];

    // create the image with proper sampling settings
    let mut texture = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        texture_data,
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    // Set nearest-neighbor filtering with proper address modes
    texture.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        mag_filter: ImageFilterMode::Nearest,
        min_filter: ImageFilterMode::Nearest,
        mipmap_filter: ImageFilterMode::Nearest,
        address_mode_u: ImageAddressMode::ClampToEdge,
        address_mode_v: ImageAddressMode::ClampToEdge,
        address_mode_w: ImageAddressMode::ClampToEdge,
        ..default()
    });

    texture
}
