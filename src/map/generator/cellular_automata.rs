use super::*;
use rand::Rng;

pub struct CellularAutomataPlugin;

impl Plugin for CellularAutomataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CellularAutomataIterations(5))
            .insert_resource(CellularAutomataRngGenBool(0.55));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct CellularAutomataIterations(pub usize);

#[derive(Resource, Deref, DerefMut)]
pub struct CellularAutomataRngGenBool(pub f64);

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}

pub fn generate(iterations: usize, rng_gen_bool: f64) -> Vec<Vec<Tile>> {
    let mut grid = initialize_grid(rng_gen_bool);

    for _ in 0..iterations {
        grid = evolve_grid(&grid);
    }

    grid.iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, &cell)| Tile {
                    grid_tile: IVec2::new(
                        navmesh_index_to_grid_tile(x),
                        navmesh_index_to_grid_tile(y),
                    ),
                    kind: match cell {
                        CellState::Alive => TileKind::Grass,
                        CellState::Dead => TileKind::Water,
                    },
                })
                .collect()
        })
        .collect()
}

fn initialize_grid(rng_gen_bool: f64) -> Vec<Vec<CellState>> {
    let mut rng = rand::thread_rng();
    let mut grid =
        vec![vec![CellState::Dead; config().grid.size as usize]; config().grid.size as usize];

    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = if rng.gen_bool(rng_gen_bool) {
                CellState::Alive
            } else {
                CellState::Dead
            };
        }
    }

    grid
}

fn evolve_grid(grid: &[Vec<CellState>]) -> Vec<Vec<CellState>> {
    let mut new_grid = grid.to_owned();

    for y in 0..config().grid.size as usize {
        for x in 0..config().grid.size as usize {
            let live_neighbors = count_live_neighbors(grid, x, y);

            new_grid[y][x] = match (grid[y][x], live_neighbors) {
                (CellState::Alive, n) if n < 4 => CellState::Dead,
                (CellState::Dead, n) if n >= 4 => CellState::Alive,
                (state, _) => state,
            };
        }
    }

    new_grid
}

fn count_live_neighbors(grid: &[Vec<CellState>], x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            #[allow(clippy::collapsible_if)]
            if nx >= 0 && nx < config().grid.size && ny >= 0 && ny < config().grid.size {
                if grid[ny as usize][nx as usize] == CellState::Alive {
                    count += 1;
                }
            }
        }
    }
    count
}
