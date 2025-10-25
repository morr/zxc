use super::*;

use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_platform::collections::HashMap;

pub fn extract_tile_noise_map(
    tile_query: &Query<&Tile>,
    noise_type: &NoiseType,
) -> HashMap<(usize, usize), f32> {
    let size = config().grid.size as usize;
    let mut noise_map = HashMap::new();

    for tile in tile_query.iter() {
        let grid_pos = tile.grid_tile;
        // Convert from grid coordinates to noise texture coordinates
        let x = grid_tile_to_navmesh_index(grid_pos.x);
        let y = grid_tile_to_navmesh_index(grid_pos.y);

        if x < size && y < size {
            noise_map.insert(
                (x, y),
                match noise_type {
                    NoiseType::Height => tile.height_noise,
                    NoiseType::Humidity => tile.humidity_noise,
                    NoiseType::Props => tile.props_noise,
                },
            );
        }
    }

    noise_map
}

pub fn render_noise_to_texture(
    noise_map: &HashMap<(usize, usize), f32>,
    noise_type: &NoiseType,
) -> Image {
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
            // convert to 0-255 for rgba
            let red_max = match noise_type {
                NoiseType::Height => 0.0,
                NoiseType::Humidity => 0.0,
                NoiseType::Props => 255.0,
            };
            let green_max = match noise_type {
                NoiseType::Height => 255.0,
                NoiseType::Humidity => 162.0,
                NoiseType::Props => 255.0,
            };
            let blue_max = match noise_type {
                NoiseType::Height => 73.0,
                NoiseType::Humidity => 255.0,
                NoiseType::Props => 255.0,
            };

            // Set RGBA values (grayscale with full opacity)
            data[texture_index] = (noise_value * red_max) as u8; // R
            data[texture_index + 1] = (noise_value * green_max) as u8; // G
            data[texture_index + 2] = (noise_value * blue_max) as u8; // B
            data[texture_index + 3] = 255; // A (full opacity)
        }
    }

    texture
}

pub fn create_blank_texture(width: u32, height: u32) -> Image {
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
