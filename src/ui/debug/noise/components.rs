use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugNoiseState {
    #[default]
    Hidden,
    Visible,
}

#[derive(Component)]
pub struct DebugNoise {
    pub is_synced: bool,
}

#[derive(Resource)]
pub struct NoiseTextureHandle(pub Handle<Image>);
