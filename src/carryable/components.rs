use super::*;

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum CarryableKind {
    Food,
}

/// On a Carryable item: the Pawn currently carrying it. Presence of this
/// component == "item is in inventory"; kind/amount stay on the item itself.
/// Despawning either side cleans the relationship up automatically.
#[derive(Component, Reflect)]
#[relationship(relationship_target = Carrying)]
pub struct CarriedBy(pub Entity);

/// On a Pawn: items it is carrying.
#[derive(Component, Reflect, Debug)]
#[relationship_target(relationship = CarriedBy)]
pub struct Carrying(Vec<Entity>);

impl Carrying {
    pub fn contains(&self, entity: Entity) -> bool {
        self.0.contains(&entity)
    }

    pub fn entities(&self) -> &[Entity] {
        &self.0
    }
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
    ) -> (Mesh2d, MeshMaterial2d<ColorMaterial>, Transform) {
        (
            Mesh2d(meshes_collection.food.clone()),
            MeshMaterial2d(assets_collection.food.clone()),
            Transform::from_translation(grid_tile.grid_tile_center_to_world().extend(ITEM_Z_INDEX)),
        )
    }

    /// Carried items are excluded from the world food stock.
    fn remove_from_stock(&self, food_stock: &mut ResMut<FoodStock>) {
        if self.kind == CarryableKind::Food {
            food_stock.amount -= self.amount;
        }
    }

    fn return_to_stock(&self, food_stock: &mut ResMut<FoodStock>) {
        if self.kind == CarryableKind::Food {
            food_stock.amount += self.amount;
        }
    }

    pub fn pick_up(
        &self,
        carryable_entity: Entity,
        pawn_entity: Entity,
        grid_tile: IVec2,
        commands: &mut Commands,
        navmesh: &mut Navmesh,
        food_stock: &mut ResMut<FoodStock>,
    ) {
        self.remove_from_stock(food_stock);

        commands
            .entity(carryable_entity)
            .insert(CarriedBy(pawn_entity))
            .remove::<Mesh2d>()
            .remove::<MeshMaterial2d<ColorMaterial>>()
            .remove::<Transform>();

        navmesh.remove_occupant::<Carryable>(&carryable_entity, grid_tile.x, grid_tile.y);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn drop_off(
        &self,
        carryable_entity: Entity,
        carried_by: Option<&CarriedBy>,
        pawn_entity: Entity,
        grid_tile: IVec2,
        commands: &mut Commands,
        assets_collection: &Res<AssetsCollection>,
        meshes_collection: &Res<MeshesCollection>,
        navmesh: &mut Navmesh,
        food_stock: &mut ResMut<FoodStock>,
    ) {
        // the item may not be carried by this pawn if the command chain
        // was interrupted before it was picked up
        if !matches!(carried_by, Some(CarriedBy(carrier)) if *carrier == pawn_entity) {
            return;
        }

        commands.entity(carryable_entity).remove::<CarriedBy>();
        self.return_to_stock(food_stock);

        let tile_occupants = navmesh
            .get_occupants::<Carryable>(grid_tile.x, grid_tile.y)
            .copied()
            .collect::<Vec<_>>();

        if tile_occupants.is_empty() {
            commands
                .entity(carryable_entity)
                .insert(Carryable::spawn_mesh_bundle(
                    grid_tile,
                    assets_collection,
                    meshes_collection,
                ));

            navmesh.add_occupant::<Carryable>(&carryable_entity, grid_tile.x, grid_tile.y);
        } else {
            commands.trigger(log_event!(MergeCarryablesEvent {
                entity_to_merge: carryable_entity,
                carryable_to_merge: self.clone(),
                grid_tile,
                merge_into_entities: tile_occupants,
            }));
        }
    }
}

#[derive(Event, Debug)]
pub struct SpawnCarryableEvent {
    pub kind: CarryableKind,
    pub amount: u32,
    pub grid_tile: IVec2,
}

#[derive(EntityEvent, Debug)]
pub struct StoreCarryableEvent {
    pub entity: Entity,
}

#[derive(EntityEvent, Debug)]
pub struct MergeCarryablesEvent {
    #[event_target]
    pub entity_to_merge: Entity,
    pub carryable_to_merge: Carryable,
    pub grid_tile: IVec2,
    pub merge_into_entities: Vec<Entity>,
}

#[derive(Resource, Default)]
pub struct FoodStock {
    pub amount: u32,
}
