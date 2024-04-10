use crate::*;
expose_submodules!(components, systems);

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(WorldState::Loading), (spawn_base, spawn_farm));
    }
}
