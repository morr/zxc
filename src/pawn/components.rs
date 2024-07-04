use bevy::utils::HashMap;
use std::ops::RangeInclusive;

use super::*;
use rand::Rng;

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Pawn {
    pub state: PawnState,

    pub age: u32,
    pub birth_year_day: u32,

    /// in seconds
    pub lifetime: f32,

    pub owned_bed: Option<Entity>,
    pub inventory: HashMap<Entity, Carryable>,
}

#[derive(Component)]
pub struct DyingMarker;

impl Default for Pawn {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let age = rng.gen_range(RangeInclusive::new(
            config().pawn.spawn_age.0,
            config().pawn.spawn_age.1,
        ));
        let lifetime = rng.gen_range(RangeInclusive::new(
            config().pawn.lifetime_span.0 as f32,
            config().pawn.lifetime_span.1 as f32,
        )) as f32
            * config().time.year_duration
            - (age as f32 * config().time.year_duration);

        Self {
            state: PawnState::Idle,
            age,
            birth_year_day: rng.gen_range(1..=config().time.days_in_year),
            lifetime,
            owned_bed: None,
            inventory: HashMap::new(),
        }
    }
}

impl Pawn {
    pub fn is_birthday(&self, total_day: u32) -> bool {
        self.birth_year_day == ElapsedTime::total_day_to_year_day(total_day)
    }

    pub fn decrease_lifetime(&mut self, amount: f32) {
        self.lifetime = f32::max(self.lifetime - amount, 0.0);
    }

    pub fn pick_up_item(
        &mut self,
        carryable_entity: Entity,
        carryable: Carryable,
        grid_tile: IVec2,
        commands: &mut Commands,
        navmesh: &mut Navmesh,
        food_stock: &mut ResMut<FoodStock>,
    ) {
        if carryable.kind == CarryableKind::Food {
            food_stock.amount -= carryable.amount;
        }

        self.inventory.insert(carryable_entity, carryable);

        commands
            .entity(carryable_entity)
            .remove::<bevy::sprite::MaterialMesh2dBundle<ColorMaterial>>();

        navmesh.remove_occupant::<Carryable>(&carryable_entity, grid_tile.x, grid_tile.y);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn drop_item(
        &mut self,
        carryable_entity: Entity,
        grid_tile: IVec2,
        commands: &mut Commands,
        assets_collection: &Res<AssetsCollection>,
        meshes_collection: &Res<MeshesCollection>,
        navmesh: &mut Navmesh,
        merge_carryables_event_writer: &mut EventWriter<MergeCarryablesEvent>,
    ) {
        // it can be not in inventory if command chain is interrupted before
        // item picked up into inventory
        let Some(carryable) = self.inventory.remove(&carryable_entity) else { return };

        let tile_occupants = navmesh
            .get_occupants::<Carryable>(grid_tile.x, grid_tile.y)
            .copied()
            .collect::<Vec<_>>();

        if !tile_occupants.is_empty() {
            merge_carryables_event_writer.send(log_event!(MergeCarryablesEvent {
                entity_to_merge: carryable_entity,
                carryable_to_merge: carryable,
                grid_tile,
                merge_into_entities: tile_occupants,
            }));
        }

        commands
            .entity(carryable_entity)
            .insert(Carryable::spawn_mesh_bundle(
                grid_tile,
                assets_collection,
                meshes_collection,
            ));

        navmesh.add_occupant::<Carryable>(&carryable_entity, grid_tile.x, grid_tile.y);
    }
}

macro_rules! pawn_states {
    (
        $( ($name:ident, $state_component_name:ident )),* $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Reflect)]
        pub enum PawnState {
            $($name),*
        }

        pub mod pawn_state {
            use bevy::prelude::*;

            $(
                #[derive(Component, Debug, Reflect)]
                pub struct $state_component_name;
            )*
        }

        impl Pawn {
            pub fn change_state(
                &mut self,
                new_state: PawnState,
                entity: Entity,
                commands: &mut Commands,
                state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<PawnState>>,
            ) -> PawnState {
                use std::mem;
                log_state_change!("Pawn({:?}).state {:?} => {:?}", entity, self.state, new_state);

                self.remove_old_state_component(commands, entity);
                let prev_state = mem::replace(&mut self.state, new_state);
                self.add_new_state_component(commands, entity);
                state_change_event_writer.send(log_event!(EntityStateChangeEvent(entity, self.state.clone())));

                prev_state
            }

            fn remove_old_state_component(&self, commands: &mut Commands, entity: Entity) {
                match &self.state {
                    $(PawnState::$name => {
                        commands.entity(entity).remove::<pawn_state::$state_component_name>();
                    },)*
                }
            }

            fn add_new_state_component(&self, commands: &mut Commands, entity: Entity) {
                match &self.state {
                    $(PawnState::$name => {
                        commands.entity(entity).insert(pawn_state::$state_component_name);
                    },)*
                }
            }

        }
    };
}

pawn_states!(
    (Idle, PawnStateIdleTag),
    (ExecutingCommand, PawnStateExecutingCommandTag),
    (Dead, PawnStateDeadTag),
);

#[derive(Component)]
pub struct PawnStateText;

#[derive(Event, Debug)]
pub struct PawnDeathEvent(pub Entity);
