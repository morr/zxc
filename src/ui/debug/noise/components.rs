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
    pub handle: Handle<Image>,
    pub is_invalid: bool
}

#[derive(Event, Debug)]
pub struct DespawnNoiseMeshEvent;
