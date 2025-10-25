use serde::{Deserialize, Serialize};
use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States, Deserialize, Serialize)]
pub enum DebugNoiseState {
    #[default]
    Hidden,
    HeightNoise,
    HumidityNoise,
    PropsNoise
}

#[derive(Component)]
pub struct DebugNoise;

#[derive(Resource)]
pub struct NoiseVisual {
    pub texture_handle: Handle<Image>,
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<ColorMaterial>,
    pub is_synced: bool
}
