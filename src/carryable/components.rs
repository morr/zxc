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
    pub fn take_into_inventory(
        &mut self,
        pawn: &mut Pawn,
        carryable_entity: Entity,
        grid_tile: IVec2,
        commands: &mut Commands,
        navmesh: &mut Navmesh,
    ) {
        pawn.inventory.insert(carryable_entity, self.clone());

        commands
            .entity(carryable_entity)
            .remove::<MaterialMesh2dBundle<ColorMaterial>>();

        navmesh.remove_occupant::<Carryable>(&carryable_entity, grid_tile.x, grid_tile.y);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn drop_from_inventory(
        &mut self,
        pawn: &mut Pawn,
        carryable_entity: Entity,
        grid_tile: IVec2,
        commands: &mut Commands,
        assets_collection: &Res<AssetsCollection>,
        meshes_collection: &Res<MeshesCollection>,
        navmesh: &mut Navmesh,
    ) {
        // it can be not in inventory if command chain is interrupted before
        // item picked up into inventory
        if pawn.inventory.remove(&carryable_entity).is_some() {
            commands
                .entity(carryable_entity)
                .insert(Carryable::spawn_mesh_bundle(
                    grid_tile,
                    assets_collection,
                    meshes_collection,
                ));

            navmesh.add_occupant::<Carryable>(carryable_entity, grid_tile.x, grid_tile.y);
        }
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
