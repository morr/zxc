use crate::*;
expose_submodules!(components, systems, farm, bed, storage);

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BedPlugin, FarmPlugin, StoragePlugin))
            .register_type::<Warehouse>()
            .register_type::<House>()
            .register_type::<Well>()
            .add_systems(
                OnExit(AppState::Loading),
                (
                    // spawn_base,
                    spawn_farm,
                    // spawn_house,
                    // spawn_well,
                    spawn_bed,
                    spawn_storage,
                ),
            );
    }
}
