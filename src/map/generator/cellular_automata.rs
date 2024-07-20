use super::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;

pub struct CellularAutomataPlugin;

impl Plugin for CellularAutomataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CellularAutomataConfig {
            auto_generate: true,
            iterations: 5,
            smoothing_iterations: 2,
            initial_alive_probability: 55,
            seed: None,
        });

        #[cfg(feature = "bevy_egui")]
        app.add_systems(Update, ui_system);
    }
}

#[derive(Resource)]
pub struct CellularAutomataConfig {
    pub auto_generate: bool,
    pub iterations: usize,
    pub smoothing_iterations: usize,
    pub initial_alive_probability: usize,
    pub seed: Option<u64>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum CellType {
    DeepWater,
    ShallowWater,
    Sand,
    Grass,
    Forest,
    Mountain,
}

pub fn generate(ca_config: &Res<CellularAutomataConfig>) -> Vec<Vec<Tile>> {
    let mut rng = match ca_config.seed {
        Some(seed) => ChaCha8Rng::seed_from_u64(seed),
        None => ChaCha8Rng::from_entropy(),
    };

    let mut grid = initialize_grid(ca_config, &mut rng);

    for _ in 0..ca_config.iterations {
        grid = evolve_grid(&grid);
    }

    for _ in 0..ca_config.smoothing_iterations {
        grid = smooth_grid(&grid);
    }

    grid.iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, &cell_type)| Tile {
                    grid_tile: IVec2::new(
                        navmesh_index_to_grid_tile(x),
                        navmesh_index_to_grid_tile(y),
                    ),
                    kind: cell_state_to_tile_kind(cell_type),
                })
                .collect()
        })
        .collect()
}

fn initialize_grid(ca_config: &CellularAutomataConfig, rng: &mut impl Rng) -> Vec<Vec<CellType>> {
    (0..config().grid.size)
        .map(|_| {
            (0..config().grid.size)
                .map(|_| {
                    if rng.gen_bool(ca_config.initial_alive_probability as f64 / 100.0) {
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
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &cell)| {
                    let live_neighbors = count_live_neighbors(grid, x, y);
                    apply_rules(cell, live_neighbors)
                })
                .collect()
        })
        .collect()
}

fn count_live_neighbors(grid: &[Vec<CellType>], x: usize, y: usize) -> usize {
    get_neighbors(grid, x, y)
        .iter()
        .filter(|&&cell| is_land(cell))
        .count()
}

fn get_neighbors(grid: &[Vec<CellType>], x: usize, y: usize) -> Vec<CellType> {
    let mut neighbors = Vec::new();
    let grid_size = grid.len() as i32;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
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
    matches!(
        cell,
        CellType::Grass | CellType::Forest | CellType::Mountain
    )
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

fn smooth_grid(grid: &[Vec<CellType>]) -> Vec<Vec<CellType>> {
    let size = grid.len();
    let mut new_grid = grid.to_owned();

    for y in 0..size {
        for x in 0..size {
            let neighbors = get_neighbors(grid, x, y);
            new_grid[y][x] = most_common_neighbor(&neighbors);
        }
    }

    new_grid
}

fn most_common_neighbor(neighbors: &[CellType]) -> CellType {
    let mut counts = HashMap::new();
    for &cell in neighbors {
        *counts.entry(cell).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(cell, _)| cell)
        .unwrap_or(CellType::DeepWater)
}

fn cell_state_to_tile_kind(cell_type: CellType) -> TileKind {
    match cell_type {
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
    mut egui_contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
    mut ca_config: ResMut<CellularAutomataConfig>,
    mut rebuild_map_event_writer: EventWriter<RebuildMapEvent>,
) {
    let ctx = egui_contexts.ctx_mut();

    bevy_egui::egui::Window::new("Cellular Automata Settings").show(ctx, |ui| {
        ui.add(bevy_egui::egui::Checkbox::new(
            &mut ca_config.auto_generate,
            "Auto Generate",
        ));
        let iterations_slider = ui.add(
            bevy_egui::egui::Slider::new(&mut ca_config.iterations, 0..=10).text("Iterations"),
        );
        let smoothing_iterations_slider = ui.add(
            bevy_egui::egui::Slider::new(&mut ca_config.smoothing_iterations, 0..=10)
                .text("Smoothing Iterations"),
        );
        let initial_alive_probability_slider = ui.add(
            bevy_egui::egui::Slider::new(&mut ca_config.initial_alive_probability, 0..=100)
                .text("Initial Alive Probability"),
        );

        let generate_new_seed_button = ui.button("Generate New Seed");
        if generate_new_seed_button.clicked() {
            ca_config.seed = Some(rand::random());
        }

        let maybe_button = if ca_config.auto_generate {
            None
        } else {
            Some(ui.button("Generate"))
        };

        let is_changed = iterations_slider.changed()
            || smoothing_iterations_slider.changed()
            || initial_alive_probability_slider.changed()
            || generate_new_seed_button.clicked();

        if (maybe_button.is_some() && maybe_button.unwrap().clicked())
            || (ca_config.auto_generate && is_changed)
        {
            rebuild_map_event_writer.send(log_event!(RebuildMapEvent {
                generator_kind: GeneratorKind::CellularAutomata
            }));
        }
    });
}
