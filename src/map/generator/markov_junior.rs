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
        None => ChaCha8Rng::from_os_rng(),
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
            if rng.random_bool(0.7) {
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
            if land_count >= 5 && rng.random_bool(0.3) {
                CellType::ShallowWater
            } else {
                cell
            }
        }
        CellType::ShallowWater => {
            if land_count >= 6 && rng.random_bool(0.4) {
                CellType::Sand
            } else {
                cell
            }
        }
        CellType::Sand => {
            if land_count >= 7 && rng.random_bool(0.5) {
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
            if fertile_count >= 3 && rng.random_bool(0.2) {
                CellType::Dirt
            } else {
                cell
            }
        }
        CellType::Dirt => {
            if fertile_count >= 2 && rng.random_bool(0.3) {
                CellType::FertileDirt
            } else if rocky_count >= 4 && rng.random_bool(0.2) {
                CellType::RockyDirt
            } else if land_count <= 3 && rng.random_bool(0.2) {
                CellType::Sand
            } else {
                cell
            }
        }
        CellType::FertileDirt => {
            if land_count <= 4 && rng.random_bool(0.2) {
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

    bevy_egui::egui::Window::new("Markov Junior Settings").show(ctx, |ui| {
        ui.add(bevy_egui::egui::Checkbox::new(
            &mut generator_config.auto_generate,
            "Auto Generate",
        ));
        let iterations_slider = ui.add(
            bevy_egui::egui::Slider::new(&mut generator_config.iterations, 0..=1000).text("Iterations"),
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


// use rand::Rng;
//
// #[derive(Clone, PartialEq, Eq, Hash)]
// struct Pattern {
//     data: Vec<char>,
//     width: usize,
//     height: usize,
// }
//
// impl Pattern {
//     fn new(data: Vec<char>, width: usize, height: usize) -> Self {
//         Pattern { data, width, height }
//     }
// }
//
// struct MarkovJunior {
//     grid: Vec<char>,
//     width: usize,
//     height: usize,
//     patterns: Vec<(Pattern, Pattern, f32)>,
// }
//
// impl MarkovJunior {
//     fn new(width: usize, height: usize) -> Self {
//         MarkovJunior {
//             grid: vec!['.'; width * height],
//             width,
//             height,
//             patterns: Vec::new(),
//         }
//     }
//
//     fn add_pattern(&mut self, input: Pattern, output: Pattern, weight: f32) {
//         self.patterns.push((input, output, weight));
//     }
//
//     fn generate(&mut self, iterations: usize) {
//         let mut rng = rand::rng();
//
//         for _ in 0..iterations {
//             let valid_patterns = self.find_valid_patterns();
//
//             if valid_patterns.is_empty() {
//                 break;
//             }
//
//             let total_weight: f32 = valid_patterns
//                 .iter()
//                 .map(|&(_, index)| self.patterns[index].2)
//                 .sum();
//             let mut choice = rng.gen::<f32>() * total_weight;
//
//             for &((x, y), index) in &valid_patterns {
//                 choice -= self.patterns[index].2;
//                 if choice <= 0.0 {
//                     let output = self.patterns[index].1.clone();
//                     self.apply_pattern(x, y, &output);
//                     break;
//                 }
//             }
//         }
//     }
//
//     fn find_valid_patterns(&self) -> Vec<((usize, usize), usize)> {
//         let mut valid_patterns = Vec::new();
//         for y in 0..self.height {
//             for x in 0..self.width {
//                 for (index, pattern) in self.patterns.iter().enumerate() {
//                     if self.pattern_fits(x, y, &pattern.0) {
//                         valid_patterns.push(((x, y), index));
//                     }
//                 }
//             }
//         }
//         valid_patterns
//     }
//
//     fn pattern_fits(&self, x: usize, y: usize, pattern: &Pattern) -> bool {
//         if x + pattern.width > self.width || y + pattern.height > self.height {
//             return false;
//         }
//
//         for py in 0..pattern.height {
//             for px in 0..pattern.width {
//                 let grid_char = self.grid[(y + py) * self.width + (x + px)];
//                 let pattern_char = pattern.data[py * pattern.width + px];
//
//                 if pattern_char != '?' && pattern_char != grid_char {
//                     return false;
//                 }
//             }
//         }
//
//         true
//     }
//
//     fn apply_pattern(&mut self, x: usize, y: usize, pattern: &Pattern) {
//         for py in 0..pattern.height {
//             for px in 0..pattern.width {
//                 let pattern_char = pattern.data[py * pattern.width + px];
//                 if pattern_char != '?' {
//                     self.grid[(y + py) * self.width + (x + px)] = pattern_char;
//                 }
//             }
//         }
//     }
//
//     fn print_grid(&self) {
//         for y in 0..self.height {
//             for x in 0..self.width {
//                 print!("{}", self.grid[y * self.width + x]);
//             }
//             println!();
//         }
//     }
// }
//
// fn main() {
//     let mut markov = MarkovJunior::new(20, 10);
//
//     // Add some example patterns
//     markov.add_pattern(
//         Pattern::new(vec!['?', '.'], 2, 1),
//         Pattern::new(vec!['#', '#'], 2, 1),
//         1.0,
//     );
//     markov.add_pattern(
//         Pattern::new(vec!['.', '?'], 2, 1),
//         Pattern::new(vec!['#', '#'], 2, 1),
//         1.0,
//     );
//     markov.add_pattern(
//         Pattern::new(vec!['?', '.', '?'], 3, 1),
//         Pattern::new(vec!['#', '#', '#'], 3, 1),
//         0.5,
//     );
//
//     markov.generate(100);
//     markov.print_grid();
// }

