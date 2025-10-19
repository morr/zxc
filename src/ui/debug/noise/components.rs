use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugNoiseState {
    #[default]
    Hidden,
    Visible,
}

#[derive(Component)]
pub struct DebugNoise;

#[derive(Resource)]
pub struct NoiseTexture {
    pub texture_handle: Handle<Image>,
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<ColorMaterial>,
    pub is_synced: bool
}
