use super::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct MarkovJuniorPlugin;

impl Plugin for MarkovJuniorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MarkovJuniorConfig {
            auto_generate: true,
            iterations: 100,
            seed: None,
        });

        #[cfg(feature = "bevy_egui")]
        app.add_systems(Update, ui_system);
    }
}

#[derive(Resource)]
pub struct MarkovJuniorConfig {
    pub auto_generate: bool,
    pub iterations: usize,
    pub seed: Option<u64>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum CellType {
    DeepWater,
    ShallowWater,
    Sand,
    RockyDirt,
    Dirt,
    FertileDirt,
}

pub fn generate(generator_config: &Res<MarkovJuniorConfig>) -> Vec<Vec<Tile>> {
    let mut rng = match generator_config.seed {
        Some(seed) => ChaCha8Rng::seed_from_u64(seed),
        None => ChaCha8Rng::from_entropy(),
    };

    let mut grid = initialize_grid(&mut rng);

    for _ in 0..generator_config.iterations {
        apply_rules(&mut grid, &mut rng);
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

fn initialize_grid(rng: &mut impl Rng) -> Vec<Vec<CellType>> {
    let mut grid = vec![vec![CellType::DeepWater; config().grid.size as usize]; config().grid.size as usize];

    // Create an initial landmass
    let center = config().grid.size / 2;
    let landmass_size = config().grid.size / 3;
    for y in (center - landmass_size)..(center + landmass_size) {
        for x in (center - landmass_size)..(center + landmass_size) {
            if rng.gen_bool(0.7) {
                grid[y as usize][x as usize] = CellType::Dirt;
            }
        }
    }

    grid
}

fn apply_rules(grid: &mut Vec<Vec<CellType>>, rng: &mut impl Rng) {
    let size = grid.len();
    let mut new_grid = grid.clone();

    for y in 0..size {
        for x in 0..size {
            let neighbors = get_neighbors(grid, x, y);
            new_grid[y][x] = apply_rule(grid[y][x], &neighbors, rng);
        }
    }

    *grid = new_grid;
}

fn get_neighbors(grid: &[Vec<CellType>], x: usize, y: usize) -> Vec<CellType> {
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

fn apply_rule(cell: CellType, neighbors: &[CellType], rng: &mut impl Rng) -> CellType {
    let land_count = neighbors.iter().filter(|&&c| c != CellType::DeepWater && c != CellType::ShallowWater).count();
    let fertile_count = neighbors.iter().filter(|&&c| c == CellType::FertileDirt).count();
    let rocky_count = neighbors.iter().filter(|&&c| c == CellType::RockyDirt).count();

    match cell {
        CellType::DeepWater => {
            if land_count >= 5 && rng.gen_bool(0.3) {
                CellType::ShallowWater
            } else {
                cell
            }
        }
        CellType::ShallowWater => {
            if land_count >= 6 && rng.gen_bool(0.4) {
                CellType::Sand
            } else {
                cell
            }
        }
        CellType::Sand => {
            if land_count >= 7 && rng.gen_bool(0.5) {
                if rocky_count >= 2 {
                    CellType::RockyDirt
                } else {
                    CellType::Dirt
                }
            } else {
                cell
            }
        }
        CellType::RockyDirt => {
            if fertile_count >= 3 && rng.gen_bool(0.2) {
                CellType::Dirt
            } else {
                cell
            }
        }
        CellType::Dirt => {
            if fertile_count >= 2 && rng.gen_bool(0.3) {
                CellType::FertileDirt
            } else if rocky_count >= 4 && rng.gen_bool(0.2) {
                CellType::RockyDirt
            } else if land_count <= 3 && rng.gen_bool(0.2) {
                CellType::Sand
            } else {
                cell
            }
        }
        CellType::FertileDirt => {
            if land_count <= 4 && rng.gen_bool(0.2) {
                CellType::Dirt
            } else {
                cell
            }
        }
    }
}

fn cell_state_to_tile_kind(cell_type: CellType) -> TileKind {
    match cell_type {
        CellType::DeepWater => TileKind::DeepWater,
        CellType::ShallowWater => TileKind::ShallowWater,
        CellType::Sand => TileKind::Sand,
        CellType::RockyDirt => TileKind::RockyDirt,
        CellType::Dirt => TileKind::Dirt,
        CellType::FertileDirt => TileKind::FertileDirt,
    }
}

#[cfg(feature = "bevy_egui")]
fn ui_system(
    mut egui_contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
    mut generator_config: ResMut<MarkovJuniorConfig>,
    mut rebuild_map_event_writer: EventWriter<RebuildMapEvent>,
) {
    let ctx = egui_contexts.ctx_mut();

    bevy_egui::egui::Window::new("Cellular Automata Settings").show(ctx, |ui| {
        ui.add(bevy_egui::egui::Checkbox::new(
            &mut generator_config.auto_generate,
            "Auto Generate",
        ));
        let iterations_slider = ui.add(
            bevy_egui::egui::Slider::new(&mut generator_config.iterations, 0..=10).text("Iterations"),
        );
        // let smoothing_iterations_slider = ui.add(
        //     bevy_egui::egui::Slider::new(&mut generator_config.smoothing_iterations, 0..=10)
        //         .text("Smoothing Iterations"),
        // );
        // let initial_alive_probability_slider = ui.add(
        //     bevy_egui::egui::Slider::new(&mut generator_config.initial_alive_probability, 0..=100)
        //         .text("Initial Alive Probability"),
        // );

        let generate_new_seed_button = ui.button("Generate New Seed");
        if generate_new_seed_button.clicked() {
            generator_config.seed = Some(rand::random());
        }

        let maybe_button = if generator_config.auto_generate {
            None
        } else {
            Some(ui.button("Generate"))
        };

        let is_changed = iterations_slider.changed()
            // || smoothing_iterations_slider.changed()
            // || initial_alive_probability_slider.changed()
            || generate_new_seed_button.clicked();

        if (maybe_button.is_some() && maybe_button.unwrap().clicked())
            || (generator_config.auto_generate && is_changed)
        {
            rebuild_map_event_writer.send(log_event!(RebuildMapEvent {
                generator_kind: GeneratorKind::MarkovJunior
            }));
        }
    });
}
