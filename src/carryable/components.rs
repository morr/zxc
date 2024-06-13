use super::*;

#[derive(Component, Reflect)]
pub struct Carryable {
    pub amount: u32,
}

#[derive(Debug, Clone, Copy, Reflect)]
pub enum ItemType {
    Food,
}

#[derive(Event, Debug)]
pub struct SpawnItemEvent {
    pub item_type: ItemType,
    pub amount: u32,
    pub grid_tile: IVec2
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct FoodStock(pub u32);
