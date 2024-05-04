use super::*;

expose_submodules!(components, systems);

pub struct FarmTilePlugin;

impl Plugin for FarmTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FarmTileProgressEvent>()
            .add_event::<EntityStateChangeEvent<FarmTileState>>()
            .add_systems(
                FixedUpdate,
                (
                    progress_on_progress_event,
                    progress_timer,
                    progress_on_state_change
                )
                    .chain()
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
