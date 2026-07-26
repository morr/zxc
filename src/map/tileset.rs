use bevy::asset::RenderAssetUsages;
use bevy::image::Image;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use super::*;

/// Array texture with one layer per `TileKind` (layer index == `kind as u16`),
/// used as the `TilemapChunk` tileset.
#[derive(Resource)]
pub struct TilesetHandle(pub Handle<Image>);

impl TileKind {
    pub fn tileset_index(self) -> u16 {
        self as u16
    }
}

/// Stacks the individual tile textures into a single array texture.
/// The order MUST match the `TileKind` enum discriminants.
pub fn build_tileset(
    mut commands: Commands,
    assets: Res<TextureAssets>,
    mut images: ResMut<Assets<Image>>,
) -> Result {
    // order == TileKind discriminant order
    let layer_handles = [
        &assets.deep_water,
        &assets.shallow_water,
        &assets.sand,
        &assets.grass,
        &assets.forest,
        &assets.mountain,
        &assets.rocky_dirt,
        &assets.dirt,
        &assets.fertile_dirt,
    ];

    // source pngs come in mixed formats (8- and 16-bit), normalize all layers
    let format = TextureFormat::Rgba8UnormSrgb;

    let first = images
        .get(layer_handles[0])
        .ok_or("tile texture not loaded")?;
    let width = first.texture_descriptor.size.width;
    let height = first.texture_descriptor.size.height;

    let mut data = Vec::with_capacity(layer_handles.len() * (width * height * 4) as usize);
    for handle in layer_handles {
        let image = images.get(handle).ok_or("tile texture not loaded")?;
        if image.texture_descriptor.size.width != width
            || image.texture_descriptor.size.height != height
        {
            return Err(format!(
                "all tile textures must share size: expected {width}x{height}, got {:?}",
                image.texture_descriptor.size
            )
            .into());
        }
        data.extend_from_slice(&rgba8_data(image)?);
    }

    let tileset = Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: layer_handles.len() as u32,
        },
        TextureDimension::D2,
        data,
        format,
        RenderAssetUsages::RENDER_WORLD,
    );

    commands.insert_resource(TilesetHandle(images.add(tileset)));
    Ok(())
}

/// Source pngs come in mixed formats; `Image::convert` cannot read 16-bit
/// sources, so squash channels to rgba8 by hand.
///
/// `Rgba16Unorm` is a LINEAR format (wgpu has no 16-bit sRGB), and that is how
/// the old per-tile sprite pipeline displayed those textures. To keep the same
/// look inside an sRGB tileset, color channels get the linear→sRGB transfer
/// applied; alpha stays linear.
fn rgba8_data(image: &Image) -> Result<Vec<u8>> {
    let data = image.data.as_ref().ok_or("tile texture has no data")?;

    match image.texture_descriptor.format {
        TextureFormat::Rgba8UnormSrgb | TextureFormat::Rgba8Unorm => Ok(data.clone()),
        TextureFormat::Rgba16Unorm => Ok(data
            .chunks_exact(8)
            .flat_map(|pixel| {
                let channel = |i: usize| {
                    u16::from_le_bytes([pixel[i * 2], pixel[i * 2 + 1]]) as f32 / u16::MAX as f32
                };
                [
                    linear_to_srgb_byte(channel(0)),
                    linear_to_srgb_byte(channel(1)),
                    linear_to_srgb_byte(channel(2)),
                    (channel(3) * 255.0).round() as u8,
                ]
            })
            .collect()),
        format => Err(format!("unsupported tile texture format {format:?}").into()),
    }
}

fn linear_to_srgb_byte(linear: f32) -> u8 {
    let srgb = if linear <= 0.003_130_8 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    };
    (srgb * 255.0).round().clamp(0.0, 255.0) as u8
}
