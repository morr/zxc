use crate::*;
expose_submodules!(components, systems);

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FarmTileProgressEvent>()
            .add_systems(
                OnExit(WorldState::Loading),
                (spawn_base, spawn_farm, spawn_house),
            )
            .add_systems(
                FixedUpdate,
                progress_farms.run_if(in_state(WorldState::Playing)),
            );
    }
}
