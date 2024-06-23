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
