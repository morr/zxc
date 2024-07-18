use super::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;

pub struct CellularAutomataPlugin;

impl Plugin for CellularAutomataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CellularAutomataConfig {
            iterations: 5,
            initial_alive_probability: 0.55,
            grid_size: 100,
            seed: None,
        });

        // #[cfg(feature = "bevy_egui")]
        // app.add_systems(Update, ui_system);
    }
}

#[derive(Resource)]
pub struct CellularAutomataConfig {
    pub iterations: usize,
    pub initial_alive_probability: f64,
    pub grid_size: usize,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellState {
    DeepWater,
    ShallowWater,
    Sand,
    Grass,
    Forest,
    Mountain,
}

pub fn generate(config: &CellularAutomataConfig) -> Vec<Vec<Tile>> {
    let mut rng = match config.seed {
        Some(seed) => ChaCha8Rng::seed_from_u64(seed),
        None => ChaCha8Rng::from_entropy(),
    };

    let mut grid = initialize_grid(config, &mut rng);

    for _ in 0..config.iterations {
        grid = evolve_grid(&grid);
    }

    // smooth_grid(&mut grid);
    for _ in 0..SMOOTHING_ITERATIONS {
        grid = smooth_grid(&grid);
    }

    grid.into_par_iter()
        .enumerate()
        .map(|(x, row)| {
            row.into_par_iter()
                .enumerate()
                .map(|(y, cell)| Tile {
                    grid_tile: IVec2::new(
                        navmesh_index_to_grid_tile(x),
                        navmesh_index_to_grid_tile(y),
                    ),
                    kind: cell_state_to_tile_kind(cell),
                })
                .collect()
        })
        .collect()
}

fn initialize_grid(config: &CellularAutomataConfig, rng: &mut impl Rng) -> Vec<Vec<CellState>> {
    (0..config.grid_size)
        .map(|_| {
            (0..config.grid_size)
                .map(|_| {
                    if rng.gen_bool(config.initial_alive_probability) {
                        CellState::Grass
                    } else {
                        CellState::DeepWater
                    }
                })
                .collect()
        })
        .collect()
}

fn evolve_grid(grid: &[Vec<CellState>]) -> Vec<Vec<CellState>> {
    let size = grid.len();
    grid.par_iter()
        .enumerate()
        .map(|(y, row)| {
            row.par_iter()
                .enumerate()
                .map(|(x, &cell)| {
                    let neighbors = count_neighbors(grid, x, y);
                    apply_rules(cell, neighbors)
                })
                .collect()
        })
        .collect()
}

fn count_neighbors(grid: &[Vec<CellState>], x: usize, y: usize) -> usize {
    let size = grid.len();
    (-1..=1)
        .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
        .filter(|&(dx, dy)| dx != 0 || dy != 0)
        .filter_map(|(dx, dy)| {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < size as i32 && ny >= 0 && ny < size as i32 {
                Some(grid[ny as usize][nx as usize])
            } else {
                None
            }
        })
        .filter(|&cell| matches!(cell, CellState::Grass | CellState::Forest | CellState::Mountain))
        .count()
}

fn apply_rules(cell: CellState, neighbors: usize) -> CellState {
    match (cell, neighbors) {
        (CellState::DeepWater, 4..=8) => CellState::ShallowWater,
        (CellState::ShallowWater, 4..=8) => CellState::Sand,
        (CellState::Sand, 4..=8) => CellState::Grass,
        (CellState::Grass, 0..=1) => CellState::Sand,
        (CellState::Grass, 5..=8) => CellState::Forest,
        (CellState::Forest, 0..=2) => CellState::Grass,
        (CellState::Forest, 7..=8) => CellState::Mountain,
        (CellState::Mountain, 0..=3) => CellState::Forest,
        (state, _) => state,
    }
}

fn smooth_grid(grid: &Vec<Vec<CellType>>) -> Vec<Vec<CellType>> {
    let mut new_grid = grid.clone();

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let neighbors = get_neighbors(grid, x, y);
            new_grid[y][x] = most_common_neighbor(&neighbors);
        }
    }

    new_grid
}

// fn smooth_grid(grid: &mut Vec<Vec<CellState>>) {
//     let size = grid.len();
//     let mut new_grid = grid.clone();
//
//     for y in 0..size {
//         for x in 0..size {
//             let mut counts = [0; 6]; // One count for each CellState variant
//             for dy in -1..=1 {
//                 for dx in -1..=1 {
//                     if dx == 0 && dy == 0 {
//                         continue;
//                     }
//                     let nx = x as i32 + dx;
//                     let ny = y as i32 + dy;
//                     if nx >= 0 && nx < size as i32 && ny >= 0 && ny < size as i32 {
//                         counts[grid[ny as usize][nx as usize] as usize] += 1;
//                     }
//                 }
//             }
//             let most_common = counts
//                 .iter()
//                 .enumerate()
//                 .max_by_key(|&(_, count)| count)
//                 .map(|(index, _)| index)
//                 .unwrap();
//             new_grid[y][x] = unsafe { std::mem::transmute(most_common as u8) };
//         }
//     }
//
//     *grid = new_grid;
// }

fn cell_state_to_tile_kind(state: CellState) -> TileKind {
    match state {
        CellState::DeepWater => TileKind::DeepWater,
        CellState::ShallowWater => TileKind::ShallowWater,
        CellState::Sand => TileKind::Sand,
        CellState::Grass => TileKind::Grass,
        CellState::Forest => TileKind::Forest,
        CellState::Mountain => TileKind::Mountain,
    }
}

// #[cfg(feature = "bevy_egui")]
// fn ui_system(
//     mut contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
//     mut config: ResMut<CellularAutomataConfig>,
//     mut rebuild_map_event_writer: EventWriter<RebuildMapEvent>,
// ) {
//     let ctx = contexts.ctx_mut();
//
//     bevy_egui::egui::Window::new("Cellular Automata Settings").show(ctx, |ui| {
//         let mut changed = false;
//         changed |= ui.add(bevy_egui::egui::Slider::new(&mut config.iterations, 0..=20).text("Iterations")).changed();
//         changed |= ui.add(bevy_egui::egui::Slider::new(&mut config.initial_alive_probability, 0.0..=1.0).text("Initial Alive Probability")).changed();
//         changed |= ui.add(bevy_egui::egui::Slider::new(&mut config.grid_size, 50..=200).text("Grid Size")).changed();
//
//         if ui.button("Generate New Seed").clicked() {
//             config.seed = Some(rand::random());
//             changed = true;
//         }
//
//         if changed {
//             rebuild_map_event_writer.send(RebuildMapEvent);
//         }
//     });
// }
