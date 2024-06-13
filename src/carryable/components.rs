use super::*;

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
