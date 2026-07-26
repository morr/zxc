use crate::prelude::*;

#[derive(Component)]
pub struct DebugNoise;

pub enum NoiseType {
    Height,
    Humidity,
    Props,
}

#[derive(Resource)]
pub struct NoiseVisuals {
    pub height_noise: NoiseVisual,
    pub humidity_noise: NoiseVisual,
    pub props_noise: NoiseVisual,
}

pub struct NoiseVisual {
    pub noise_type: NoiseType,
    pub texture_handle: Handle<Image>,
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<ColorMaterial>,
    pub is_synced: bool,
}
