use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use super::*;

pub struct DebugNoisePlugin;
impl Plugin for DebugNoisePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(DebugNoiseState::Hidden)
            .add_event::<StateChangeEvent<DebugNoiseState>>()
            .add_systems(Startup, setup_noise_texture.after(generate_noise))
            .add_systems(
                FixedUpdate,
                handle_state_changes.run_if(in_state(AppState::Playing)),
            );

        if config().debug.is_noise {
            app.add_systems(
                OnExit(AppState::Loading),
                (|mut next_state: ResMut<NextState<DebugNoiseState>>,
                  mut debug_noise_state_change_event_writer: EventWriter<
                    StateChangeEvent<DebugNoiseState>,
                >| {
                    next_state.set(DebugNoiseState::Visible);
                    debug_noise_state_change_event_writer
                        .send(log_event!(StateChangeEvent(DebugNoiseState::Visible)));
                })
                .after(generate_map),
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

fn setup_noise_texture(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let texture = create_empty_texture(config().grid.size as u32, config().grid.size as u32);
    let handle = images.add(texture);

    commands.insert_resource(NoiseTextureHandle(handle));
}

fn handle_state_changes(
    mut commands: Commands,
    mut event_reader: EventReader<StateChangeEvent<DebugNoiseState>>,
    query_mesh: Query<Entity, With<DebugNoise>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    noise_texture: Res<NoiseTextureHandle>,
) {
    for StateChangeEvent(state) in event_reader.read() {
        match state {
            DebugNoiseState::Visible => {
                println!("DebugNoiseState::Hidden => DebugNoiseState::Visible");
                let grid_world_size = config().grid.size as f32 * config().tile.size;
                let mesh = meshes.add(Rectangle::new(grid_world_size, grid_world_size));
                let material = materials.add(ColorMaterial::from(noise_texture.0.clone()));

                commands.spawn((
                    Mesh2d(mesh),
                    MeshMaterial2d(material),
                    Transform::from_xyz(0.0, 0.0, TILE_Z_INDEX + 1.0),
                    DebugNoise,
                ));

                // println!("DebugNoiseState::Visible => DebugNoiseState::Hidden");

                // commands.spawn((
                //     Mesh2d(mesh_handle.clone()),
                //     MeshMaterial2d(if navtile.is_passable() {
                //         assets.navmesh_passable.clone()
                //     } else {
                //         assets.navmesh_impassable.clone()
                //     }),
                //     Transform::from_xyz(0., 0., TILE_Z_INDEX + 1.0),
                //     DebugNoiseMesh
                //
                //     // Sprite::from_image(
                //     //     noise_texture.0.clone()
                //     // ),
                //     // Transform::from_xyz(0., 0., TILE_Z_INDEX + 1.0),
                // ));
            }
            DebugNoiseState::Hidden => {
                println!("DebugNoiseState::Visible => DebugNoiseState::Hidden");
                commands.entity(query_mesh.single()).despawn_recursive();
            }
        }
    }
}

fn create_empty_texture(width: u32, height: u32) -> Image {
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
