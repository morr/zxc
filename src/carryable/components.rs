use super::*;

use bevy::sprite::MaterialMesh2dBundle;

#[derive(Debug, Clone, Copy, Reflect)]
pub enum CarryableKind {
    Food,
}

#[derive(Component, Reflect)]
pub struct Carryable {
    pub kind: CarryableKind,
    pub amount: u32,
}

impl Carryable {
    pub fn pick_up_by(
        &mut self,
        pawn: &mut Pawn,
        carryable_entity: Entity,
        commands: &mut Commands,
    ) {
        assert!(pawn.carried_item.is_none(), "Pawn is already carrying an item!");
        pawn.carried_item = Some(carryable_entity);

        commands
            .entity(carryable_entity)
            .remove::<MaterialMesh2dBundle<ColorMaterial>>();
    }
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
