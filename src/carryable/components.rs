use super::*;

use bevy::sprite::MaterialMesh2dBundle;

#[derive(Debug, Clone, Copy, Reflect)]
pub enum CarryableKind {
    Food,
}

#[derive(Component, Reflect, Debug, Clone)]
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
        pawn.inventory.insert(carryable_entity, self.clone());

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
