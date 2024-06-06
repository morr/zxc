use crate::*;

pub struct FeedablePlugin;

impl Plugin for FeedablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Feedable>().add_systems(
            Update,
            progress_saturation
                .run_if(in_state(AppState::Playing))
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

const FULL_SATURATION: f32 = 100.;
const EMPTY_SATURATION: f32 = 0.;

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Feedable {
    pub saturation: f32,
}

impl Default for Feedable {
    fn default() -> Self {
        Self {
            saturation: FULL_SATURATION,
        }
    }
}

