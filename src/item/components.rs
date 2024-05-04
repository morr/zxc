use super::*;

#[derive(Component)]
pub struct FoodItem;

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Food,
}

#[derive(Event, Debug)]
pub struct SpawnItemEvent {
    pub item_type: ItemType,
    pub amount: usize,
}
