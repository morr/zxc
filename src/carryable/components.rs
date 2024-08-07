use super::*;

use bevy::sprite::MaterialMesh2dBundle;

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum CarryableKind {
    Food,
    InInventory,
}

#[derive(Component, Reflect, Default)]
pub struct CarryableFoodMarker;

#[derive(Component, Reflect, Debug, Clone)]
pub struct Carryable {
    pub kind: CarryableKind,
    pub amount: u32,
}

impl Carryable {
    pub fn spawn_mesh_bundle(
        grid_tile: IVec2,
        assets_collection: &Res<AssetsCollection>,
        meshes_collection: &Res<MeshesCollection>,
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        MaterialMesh2dBundle {
            mesh: meshes_collection.food.clone().into(),
            material: assets_collection.food.clone(),
            transform: Transform::from_translation(
                grid_tile.grid_tile_center_to_world().extend(ITEM_Z_INDEX),
            ),
            ..default()
        }
    }

    pub fn to_inventory(&mut self, food_stock: &mut ResMut<FoodStock>) {
        if self.kind == CarryableKind::Food {
            food_stock.amount -= self.amount;
        }

        self.kind = CarryableKind::InInventory;
        self.amount = 0;
    }

    pub fn from_inventory(&mut self, kind: CarryableKind,  amount: u32, food_stock: &mut ResMut<FoodStock>) {
        self.kind = kind;
        self.amount = amount;

        if self.kind == CarryableKind::Food {
            food_stock.amount += self.amount;
        }
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

#[derive(Event, Debug)]
pub struct MergeCarryablesEvent {
    pub entity_to_merge: Entity,
    pub carryable_to_merge: Carryable,
    pub grid_tile: IVec2,
    pub merge_into_entities: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct FoodStock {
    pub amount: u32,
}
