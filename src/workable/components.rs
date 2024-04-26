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

impl Workable {
    pub fn perform_work(&mut self, elapsed_time: f32) {
        self.progress += elapsed_time;
    }
}
