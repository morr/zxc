use std::any::TypeId;

use super::*;
use bevy::utils::{HashMap, HashSet};
use rand_distr::num_traits::Zero;

const DEFAULT_COST: i32 = 1;
pub const COST_MULTIPLIER: f32 = 100.0;

const INITIAL_NAV_COST: i32 = (DEFAULT_COST as f32 * COST_MULTIPLIER) as i32;

#[derive(Debug)]
pub struct Navtile {
    pub cost: Option<i32>,
    pub occupied_by: HashMap<TypeId, HashSet<Entity>>,
}

impl Navtile {
    fn new() -> Self {
        Self {
            cost: Some(INITIAL_NAV_COST),
            occupied_by: HashMap::new(),
        }
    }

    pub fn is_passable(&self) -> bool {
        self.cost.is_some()
    }

    pub fn add_occupant<T: 'static>(&mut self, entity: Entity) {
        let type_id = TypeId::of::<T>();

        self.occupied_by.entry(type_id).or_default().insert(entity);
    }

    pub fn remove_occupant<T: 'static>(&mut self, entity: &Entity) {
        if let Some(entities) = self.occupied_by.get_mut(&TypeId::of::<T>()) {
            entities.remove(entity);

            // not sure if it is necessary to cleanup type T from HashMap
            // if entities.is_empty() {
            //     self.occupied_by.remove(&TypeId::of::<T>());
            // }
        }
    }

    pub fn get_occupants<T: 'static>(&self) -> impl Iterator<Item = &Entity> {
        self.occupied_by
            .get(&TypeId::of::<T>())
            .into_iter()
            .flat_map(|set| set.iter())
    }

    pub fn get_all_occupants(&self) -> impl Iterator<Item = &Entity> {
        self.occupied_by.values().flat_map(|set| set.iter())
    }

    pub fn has_occupants_except_of<T: 'static>(&self) -> bool {
        self.occupied_by.iter().any(|(&type_id, entities)| {
            type_id != TypeId::of::<T>() && !entities.is_empty()
        })
    }

    pub fn config_cost_to_pathfinding_cost(config_cost: f32) -> Option<i32> {
        if config_cost.is_zero() {
            None
        } else {
            Some((1.0 / config_cost * COST_MULTIPLIER) as i32)
        }
    }
}

pub struct Navtiles(pub Vec<Vec<Navtile>>);

impl Default for Navtiles {
    fn default() -> Self {
        Self(
            (0..config().grid.size)
                .map(|_x| (0..config().grid.size).map(|_y| Navtile::new()).collect())
                .collect(),
        )
    }
}

impl Navtiles {
    pub fn get(&self, grid_tile_x: i32, grid_tile_y: i32) -> &Navtile {
        &self.0[grid_tile_to_navmesh_index(grid_tile_x)][grid_tile_to_navmesh_index(grid_tile_y)]
    }

    pub fn get_mut(&mut self, grid_tile_x: i32, grid_tile_y: i32) -> &mut Navtile {
        &mut self.0[grid_tile_to_navmesh_index(grid_tile_x)]
            [grid_tile_to_navmesh_index(grid_tile_y)]
    }

    pub fn get_some(&self, grid_tile_x: i32, grid_tile_y: i32) -> Option<&Navtile> {
        self.0
            .get(grid_tile_to_navmesh_index(grid_tile_x))?
            .get(grid_tile_to_navmesh_index(grid_tile_y))
    }

    pub fn get_passable(&self, grid_tile_x: i32, grid_tile_y: i32) -> Option<&Navtile> {
        self.get_some(grid_tile_x, grid_tile_y)
            .filter(|navtile| navtile.is_passable())
    }

    pub fn for_each_tile_mut<F>(&self, mut lambda: F)
    where
        F: FnMut(&Navtile, IVec2),
    {
        for (x, row) in self.0.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                lambda(
                    tile,
                    IVec2::new(navmesh_index_to_grid_tile(x), navmesh_index_to_grid_tile(y)),
                );
            }
        }
    }
}
