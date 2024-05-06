use super::*;

expose_submodules!(components, systems);

pub struct FarmPlugin;

impl Plugin for FarmPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Farm>()
            .add_event::<FarmProgressEvent>()
            .add_event::<FarmTendedEvent>()
            .add_event::<EntityStateChangeEvent<FarmState>>()
            .add_systems(
                FixedUpdate,
                (
                    progress_on_farm_progress_event,
                    progress_on_farm_tended_event,
                    progress_planted_timer,
                    progress_harvested_timer,
                    progress_on_state_changed,
                    progress_on_new_day,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
