use super::*;
use bevy::utils::HashSet;
use std::any::TypeId;

#[derive(Debug)]
pub struct Navtile {
    pub cost: Option<i32>,
    pub occupied_by: HashSet<EntityWithComponent>,
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

    pub fn add_entity<T: 'static>(&mut self, entity: Entity) {
        self.occupied_by
            .insert(EntityWithComponent::new::<T>(entity));
    }

    pub fn remove_entity(&mut self, id: &Entity) {
        self.occupied_by.retain(|e| e.id != *id);
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct EntityWithComponent {
    pub id: Entity,
    pub component_type: TypeId,
}

impl EntityWithComponent {
    pub fn new<T: 'static>(id: Entity) -> Self {
        Self {
            id,
            component_type: TypeId::of::<T>(),
        }
    }
}

pub struct Navtiles(pub Vec<Vec<Navtile>>);

impl Default for Navtiles {
    fn default() -> Self {
        Self(
            (0..CONFIG.grid.size)
                .map(|_x| (0..CONFIG.grid.size).map(|_y| Navtile::new()).collect())
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
