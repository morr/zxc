use crate::*;

pub struct HungerablePlugin;

impl Plugin for HungerablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Hungerable>();
            // .add_event::<RestCompleteEvent>()
            // .add_systems(
            //     Update,
            //     progress_stamina
            //         .run_if(in_state(AppState::Playing))
            //         .run_if(in_state(SimulationState::Running)),
            // );
    }
}

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Hungerable {
    pub hunger: f32,
}
