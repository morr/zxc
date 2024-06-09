use std::{
    ops::Range,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use super::*;

#[derive(Resource, Default)]
pub struct ArcNavmesh(pub Arc<RwLock<Navmesh>>);

impl ArcNavmesh {
    pub fn read(&self) -> RwLockReadGuard<Navmesh> {
        self.0.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<Navmesh> {
        self.0.write().unwrap()
    }
}

pub struct Navmesh {
    pub navtiles: Navtiles,
    successors: Vec<Vec<Vec<(IVec2, i32)>>>,
}

impl Default for Navmesh {
    fn default() -> Self {
        let navtiles = Navtiles::default();
        let successors = generate_successors(&navtiles);

        Self {
            navtiles,
            successors,
        }
    }
}

impl Navmesh {
    pub fn tile_successors(&self, x: i32, y: i32) -> Vec<(IVec2, i32)> {
        self.successors[grid_tile_to_navmesh_index(x)][grid_tile_to_navmesh_index(y)].clone()
    }

    pub fn update_cost(&mut self, x_range: Range<i32>, y_range: Range<i32>, cost: Option<i32>) {
        for x in x_range.clone() {
            for y in y_range.clone() {
                self.navtiles.get_mut(x, y).cost = cost;
            }
        }

        // regenerate successors
        for x in (x_range.start - 1)..(x_range.end + 1) {
            for y in (y_range.start - 1)..(y_range.end + 1) {
                self.successors[grid_tile_to_navmesh_index(x)][grid_tile_to_navmesh_index(y)] =
                    tile_successors(x, y, &self.navtiles);
            }
        }
    }

    pub fn is_passable(&self, x: i32, y: i32) -> bool {
        self.navtiles
            .get_some(x, y)
            .map_or(false, |navtile| navtile.is_passable())
    }

    pub fn add_occupation<T: 'static>(&mut self, id: Entity, grid_tile_x: i32, grid_tile_y: i32) {
        self.navtiles
            .get_mut(grid_tile_x, grid_tile_y)
            .add_occupation::<T>(id);
    }

    pub fn remove_occupation<T: 'static>(&mut self, id: &Entity, grid_tile_x: i32, grid_tile_y: i32) {
        self.navtiles
            .get_mut(grid_tile_x, grid_tile_y)
            .remove_occupation::<T>(id);
    }

    pub fn get_type_occupants<T: 'static>(
        &self,
        grid_tile_x: i32,
        grid_tile_y: i32,
    ) -> impl Iterator<Item = &Entity> {
        self.navtiles
            .get(grid_tile_x, grid_tile_y)
            .get_type_occupants::<T>()
    }

    pub fn get_all_occupants(
        &self,
        grid_tile_x: i32,
        grid_tile_y: i32,
    ) -> impl Iterator<Item = &Entity> {
        self.navtiles
            .get(grid_tile_x, grid_tile_y)
            .get_all_occupants()
    }

    pub fn has_occupants_except_of<T: 'static>(
        &self,
        grid_tile_x: i32,
        grid_tile_y: i32,
    ) -> bool {
        self.navtiles
            .get(grid_tile_x, grid_tile_y)
            .has_occupants_except_of::<T>()
    }
}

fn generate_successors(navtiles: &Navtiles) -> Vec<Vec<Vec<(IVec2, i32)>>> {
    navtiles
        .0
        .iter()
        .enumerate()
        .map(|(x, col)| {
            col.iter()
                .enumerate()
                .map(|(y, navtile)| {
                    if navtile.is_passable() {
                        tile_successors(
                            navmesh_index_to_grid_tile(x),
                            navmesh_index_to_grid_tile(y),
                            navtiles,
                        )
                    } else {
                        Vec::new()
                    }
                })
                .collect()
        })
        .collect()
}

fn tile_successors(x: i32, y: i32, navtiles: &Navtiles) -> Vec<(IVec2, i32)> {
    [
        (x - 1, y),     // left
        (x - 1, y - 1), // left-top
        (x, y - 1),     // top
        (x + 1, y - 1), // top-right
        (x + 1, y),     // right
        (x + 1, y + 1), // right-bottom
        (x, y + 1),     // bottom
        (x - 1, y + 1), // bottom-left
    ]
    .iter()
    .filter_map(|&(nx, ny)| {
        let is_diagonal_movable = x != nx && y != ny;

        navtiles.get_passable(nx, ny).and_then(|navtile| {
            if !is_diagonal_movable
                // check that both adjacent tiles are passable
                || (navtiles.get_passable(x, ny).is_some()
                    && navtiles.get_passable(nx, y).is_some())
            {
                let tile_cost = navtile.cost.unwrap();
                Some((
                    IVec2 { x: nx, y: ny },
                    if is_diagonal_movable {
                        // this is not strictly correct calculation
                        // instead of cost * sqrt(2) it should be
                        // (tile1.cost + sqrt(2))/2 + (tile2.cost + sqrt(2))/2
                        (tile_cost as f32 * f32::sqrt(2.0)).floor() as i32
                    } else {
                        tile_cost
                    },
                ))
            } else {
                None
            }
        })
    })
    .collect()
}
