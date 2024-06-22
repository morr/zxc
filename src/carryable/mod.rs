use crate::*;

expose_submodules!( systems);

pub struct CarryablePlugin;

impl Plugin for CarryablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Carryable>()
            .init_resource::<FoodStock>()
            .add_event::<SpawnCarryableEvent>()
            .add_systems(OnExit(AppState::Loading), spawn_initial_items)
            .add_systems(
                FixedUpdate,
                spawn_on_event.run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Component, Reflect)]
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
    pub grid_tile: IVec2
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct FoodStock(pub u32);

