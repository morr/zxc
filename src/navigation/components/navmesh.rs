use super::*;

#[derive(Resource)]
pub struct Navmesh {
    pub navtiles: Navtiles,
    successors: Vec<Vec<Vec<(IVec2, i32)>>>,
}

impl Default for Navmesh {
    fn default() -> Self {
        let navtiles = Navtiles::default();
        let successors = generate_successors(&navtiles);

        // println!("{:?}", successors[0][0].len());

        Self {
            navtiles,
            successors,
        }
    }
}

impl Navmesh {
    pub fn tile_successors(&self, x: i32, y: i32) -> Vec<(IVec2, i32)> {
        self.successors[(x + GRID_COLS_HALF) as usize][(y + GRID_ROWS_HALF) as usize].clone()
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
                    if navtile.passable {
                        tile_successors(
                            x as i32 - GRID_COLS_HALF,
                            y as i32 - GRID_ROWS_HALF,
                            navtiles,
                        )
                    } else {
                        Vec::new()
                    }
                })
                .collect::<Vec<Vec<(IVec2, i32)>>>()
        })
        .collect::<Vec<Vec<Vec<(IVec2, i32)>>>>()
}

fn tile_successors(x: i32, y: i32, navtiles: &Navtiles) -> Vec<(IVec2, i32)> {
    [
        (x - 1, y),     // left
        (x - 1, y - 1), // left-top
        (x, y - 1),     // top
        (x + 1, y - 1), // top-right
        (x + 1, y),     // right
        (x + 1, y + 1), // right-bototm
        (x, y + 1),     // bottom
        (x - 1, y + 1), // bottom-left
    ]
    .iter()
    .filter_map(|&(nx, ny)| {
        navtiles.get_if_passable(nx, ny).and_then(|navtile| {
            let is_diagonal_movement = x != nx && y != ny;

            if !is_diagonal_movement
                // check that both adjacent tiles are passable
                || (navtiles.get_if_passable(x, ny).is_some()
                    && navtiles.get_if_passable(nx, y).is_some())
            {
                Some((
                    IVec2 { x: nx, y: ny },
                    if is_diagonal_movement {
                        // this is not strictly correct calculation
                        // instead of cost * sqrt(2) it should be
                        // (tile1.cost + sqrt(2))/2 + (tile2.cost + sqrt(2))/2
                        (navtile.cost as f32 * f32::sqrt(2.0)).floor() as i32
                    } else {
                        navtile.cost
                    },
                ))
            } else {
                None
            }
        })
    })
    .collect::<Vec<_>>()
}