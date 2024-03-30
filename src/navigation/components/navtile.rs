use super::*;

#[derive(Debug)]
pub struct Navtile {
    // pub x: i32,
    // pub y: i32,
    pub cost: i32,
    // pub occupied_by: HashSet<Entity>,
    pub passable: bool,
}

impl Navtile {
    // fn new(x: i32, y: i32) -> Self {
    fn new() -> Self {
        Self {
            // x,
            // y,
            cost: INITIAL_NAV_COST,
            // occupied_by: HashSet::default(),
            passable: INITIAL_PASSABLE,
        }
    }
}

pub struct Navtiles(pub Vec<Vec<Navtile>>);

impl Default for Navtiles {
    fn default() -> Self {
        Self(
            (0..GRID_COLS)
                .map(|_x| {
                    (0..GRID_ROWS)
                        .map(|_y| Navtile::new())
                        .collect::<Vec<Navtile>>()
                })
                .collect::<Vec<Vec<Navtile>>>(),
        )
    }
}

impl Navtiles {
    pub fn get(&self, x: i32, y: i32) -> Option<&Navtile> {
        self.0
            .get((x + GRID_COLS_HALF) as usize)?
            .get((y + GRID_ROWS_HALF) as usize)
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Navtile> {
        self.0
            .get_mut(grid_tile_to_navmesh_index(x))?
            .get_mut(grid_tile_to_navmesh_index(y))
    }

    pub fn get_if_passable(&self, x: i32, y: i32) -> Option<&Navtile> {
        let result = self.get(x, y);

        if result?.passable {
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
