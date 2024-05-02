use crate::*;
expose_submodules!(components, systems, farm_tile);

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FarmTileProgressEvent>()
            .add_systems(
                OnExit(AppState::Loading),
                (spawn_base, spawn_farm, spawn_house, spawn_well),
            )
            .add_systems(
                FixedUpdate,
                progress_farms.run_if(in_state(AppState::Playing)),
            );
    }
}
