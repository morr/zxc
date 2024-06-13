use super::*;

#[derive(Component, Reflect)]
pub struct Item {
    pub amount: u32,
}

#[derive(Debug, Clone, Copy)]
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
