use super::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use std::collections::HashMap;

pub struct CellularAutomataPlugin;

impl Plugin for CellularAutomataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CellularAutomataConfig {
            iterations: 5,
            initial_alive_probability: 0.55,
            grid_size: 100,
            seed: None,
        });

        #[cfg(feature = "bevy_egui")]
        app.add_systems(Update, ui_system);
    }
}

#[derive(Resource)]
pub struct CellularAutomataConfig {
    pub iterations: usize,
    pub initial_alive_probability: f64,
    pub grid_size: usize,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CellType {
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

    smooth_grid(&mut grid);

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
                    kind: cell_type_to_tile_kind(cell),
                })
                .collect()
        })
        .collect()
}

fn initialize_grid(config: &CellularAutomataConfig, rng: &mut impl Rng) -> Vec<Vec<CellType>> {
    (0..config.grid_size)
        .map(|_| {
            (0..config.grid_size)
                .map(|_| {
                    if rng.gen_bool(config.initial_alive_probability) {
                        CellType::Grass
                    } else {
                        CellType::DeepWater
                    }
                })
                .collect()
        })
        .collect()
}

fn evolve_grid(grid: &[Vec<CellType>]) -> Vec<Vec<CellType>> {
    let size = grid.len();
    grid.par_iter()
        .enumerate()
        .map(|(y, row)| {
            row.par_iter()
                .enumerate()
                .map(|(x, &cell)| {
                    let live_neighbors = count_live_neighbors(grid, x, y);
                    apply_rules(cell, live_neighbors)
                })
                .collect()
        })
        .collect()
}

fn count_live_neighbors(grid: &Vec<Vec<CellType>>, x: usize, y: usize) -> usize {
    get_neighbors(grid, x, y).iter().filter(|&&cell| is_land(cell)).count()
}

fn get_neighbors(grid: &Vec<Vec<CellType>>, x: usize, y: usize) -> Vec<CellType> {
    let mut neighbors = Vec::new();
    let grid_size = grid.len() as i32;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < grid_size && ny >= 0 && ny < grid_size {
                neighbors.push(grid[ny as usize][nx as usize]);
            }
        }
    }
    neighbors
}

fn is_land(cell: CellType) -> bool {
    matches!(cell, CellType::Grass | CellType::Forest | CellType::Mountain)
}

fn apply_rules(cell: CellType, live_neighbors: usize) -> CellType {
    match (cell, live_neighbors) {
        (CellType::DeepWater, 4..=8) => CellType::ShallowWater,
        (CellType::ShallowWater, 4..=8) => CellType::Sand,
        (CellType::Sand, 4..=8) => CellType::Grass,
        (CellType::Grass, 0..=1) => CellType::Sand,
        (CellType::Grass, 5..=8) => CellType::Forest,
        (CellType::Forest, 0..=2) => CellType::Grass,
        (CellType::Forest, 7..=8) => CellType::Mountain,
        (CellType::Mountain, 0..=3) => CellType::Forest,
        (state, _) => state,
    }
}

fn smooth_grid(grid: &mut Vec<Vec<CellType>>) {
    let size = grid.len();
    let mut new_grid = grid.clone();

    for y in 0..size {
        for x in 0..size {
            let neighbors = get_neighbors(grid, x, y);
            new_grid[y][x] = most_common_neighbor(&neighbors);
        }
    }

    *grid = new_grid;
}

fn most_common_neighbor(neighbors: &[CellType]) -> CellType {
    let mut counts = HashMap::new();
    for &cell in neighbors {
        *counts.entry(cell).or_insert(0) += 1;
    }
    counts.into_iter().max_by_key(|&(_, count)| count).map(|(cell, _)| cell).unwrap_or(CellType::DeepWater)
}

fn cell_type_to_tile_kind(cell: CellType) -> TileKind {
    match cell {
        CellType::DeepWater => TileKind::DeepWater,
        CellType::ShallowWater => TileKind::ShallowWater,
        CellType::Sand => TileKind::Sand,
        CellType::Grass => TileKind::Grass,
        CellType::Forest => TileKind::Forest,
        CellType::Mountain => TileKind::Mountain,
    }
}

#[cfg(feature = "bevy_egui")]
fn ui_system(
    mut contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
    mut config: ResMut<CellularAutomataConfig>,
    mut rebuild_map_event_writer: EventWriter<RebuildMapEvent>,
) {
    let ctx = contexts.ctx_mut();

    bevy_egui::egui::Window::new("Cellular Automata Settings").show(ctx, |ui| {
        let mut changed = false;
        changed |= ui.add(bevy_egui::egui::Slider::new(&mut config.iterations, 0..=20).text("Iterations")).changed();
        changed |= ui.add(bevy_egui::egui::Slider::new(&mut config.initial_alive_probability, 0.0..=1.0).text("Initial Alive Probability")).changed();
        changed |= ui.add(bevy_egui::egui::Slider::new(&mut config.grid_size, 50..=200).text("Grid Size")).changed();

        if ui.button("Generate New Seed").clicked() {
            config.seed = Some(rand::random());
            changed = true;
        }

        if changed {
            rebuild_map_event_writer.send(RebuildMapEvent);
        }
    });
}
