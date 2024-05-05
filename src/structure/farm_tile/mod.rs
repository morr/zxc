use super::*;

expose_submodules!(components, systems);

pub struct FarmTilePlugin;

impl Plugin for FarmTilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<FarmTile>()
            .add_event::<FarmTileProgressEvent>()
            .add_event::<FarmTileTendedEvent>()
            .add_event::<EntityStateChangeEvent<FarmTileState>>()
            .add_systems(
                FixedUpdate,
                (
                    progress_on_progress_event,
                    progress_on_tending_event,
                    progress_planted_timer,
                    progress_harvested_timer,
                    progress_on_state_changed,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
