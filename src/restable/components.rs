use super::*;

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Restable {
    pub stamina: f32,
}

impl Default for Restable {
    fn default() -> Self {
        Self {
            stamina: 100.0
        }
    }
}
