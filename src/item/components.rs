use super::*;

#[derive(Component, Reflect)]
pub struct FoodItem {
    pub amount: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Food,
}

#[derive(Event, Debug)]
pub struct SpawnItemEvent {
    pub item_type: ItemType,
    pub amount: usize,
    pub grid_tile: IVec2
}
