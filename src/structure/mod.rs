use crate::*;
expose_submodules!(components, systems, farm_tile);

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FarmTileProgressEvent>()
            .add_event::<EntityStateChangeEvent<FarmTileState>>()
            .add_systems(
                OnExit(AppState::Loading),
                (spawn_base, spawn_farm, spawn_house, spawn_well),
            )
            .add_systems(
                FixedUpdate,
                progress_farm_tile_state.run_if(in_state(AppState::Playing))
            )
            .add_systems(
                FixedUpdate,
                progress_farm_tile_timer
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running))
            );

    }
}
