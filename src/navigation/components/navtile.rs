use bevy::utils::HashSet;

use super::*;

#[derive(Debug)]
pub struct Navtile {
    pub cost: Option<i32>,
    pub occupied_by: HashSet<Entity>,
}

impl Navtile {
    fn new() -> Self {
        Self {
            cost: Some(INITIAL_NAV_COST),
            occupied_by: HashSet::default(),
        }
    }

    pub fn is_passable(&self) -> bool {
        self.cost.is_some()
    }

    pub fn place_entity(&mut self, id: Entity) {
        self.occupied_by.insert(id);
    }
}

pub struct Navtiles(pub Vec<Vec<Navtile>>);

impl Default for Navtiles {
    fn default() -> Self {
        Self(
            (0..CONFIG.grid.size)
                .map(|_x| {
                    (0..CONFIG.grid.size)
                        .map(|_y| Navtile::new())
                        .collect::<Vec<Navtile>>()
                })
                .collect::<Vec<Vec<Navtile>>>(),
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

    // methods with bounds check. this should never happen so I'm fine with rust panicking on
    // invalid access for now
    // pub fn get(&self, grid_tile_x: i32, grid_tile_y: i32) -> Option<&Navtile> {
    //     let x_index = grid_tile_to_navmesh_index(grid_tile_x);
    //     let y_index = grid_tile_to_navmesh_index(grid_tile_y);
    //
    //     if x_index < self.0.len() && y_index < self.0[x_index].len() {
    //         Some(&self.0[x_index][y_index])
    //     } else {
    //         None
    //     }
    // }
    // pub fn get_mut(&mut self, grid_tile_x: i32, grid_tile_y: i32) -> Option<&mut Navtile> {
    //     let x_index = grid_tile_to_navmesh_index(grid_tile_x);
    //     let y_index = grid_tile_to_navmesh_index(grid_tile_y);
    //
    //     if x_index < self.0.len() && y_index < self.0[x_index].len() {
    //         Some(&mut self.0[x_index][y_index])
    //     } else {
    //         None
    //     }
    // }

    pub fn get_some(&self, grid_tile_x: i32, grid_tile_y: i32) -> Option<&Navtile> {
        self.0
            .get(grid_tile_to_navmesh_index(grid_tile_x))?
            .get(grid_tile_to_navmesh_index(grid_tile_y))
    }

    pub fn get_passable(&self, grid_tile_x: i32, grid_tile_y: i32) -> Option<&Navtile> {
        let result = self.get_some(grid_tile_x, grid_tile_y);

        if result?.is_passable() {
            result
        } else {
            None
        }
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
