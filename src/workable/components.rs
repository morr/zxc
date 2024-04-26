use super::*;

#[derive(Debug, Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Workable {
    progress: f32,
}

impl Default for Workable {
    fn default() -> Self {
        Self { progress: 0.0 }
    }
}
