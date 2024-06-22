use crate::*;

expose_submodules!(systems);

pub struct CarryablePlugin;

impl Plugin for CarryablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Carryable>()
            .init_resource::<FoodStock>()
            .add_event::<SpawnCarryableEvent>()
            .add_event::<StoreCarryableEvent>()
            .add_systems(OnExit(AppState::Loading), spawn_initial_items)
            .add_systems(
                FixedUpdate,
                (spawn_on_event, store_on_event)
                    .chain()
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}

#[derive(Component, Reflect, Debug)]
pub struct Carryable {
    pub kind: CarryableKind,
    pub amount: u32,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum CarryableKind {
    Food,
}

#[derive(Event, Debug)]
pub struct SpawnCarryableEvent {
    pub kind: CarryableKind,
    pub amount: u32,
    pub grid_tile: IVec2,
}

#[derive(Event, Debug)]
pub struct StoreCarryableEvent {
    pub entity: Entity,
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct FoodStock(pub u32);
