use super::*;

#[derive(Debug, Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Workable {
    /// in seconds
    pub work_amount_done: f32,
    /// in seconds
    pub work_amount_total: f32,
}

impl Workable {
    pub fn new(work_amount_total: f32) -> Self {
        Self {
            work_amount_total,
            work_amount_done: 0.0,
        }
    }
}

impl Workable {
    pub fn perform_work(&mut self, elapsed_time: f32) {
        self.work_amount_done += elapsed_time * CONFIG.pawn.work_force;
    }
}
