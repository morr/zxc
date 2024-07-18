use super::*;
use rand::Rng;

pub struct CellularAutomataPlugin;

impl Plugin for CellularAutomataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CellularAutomataIterations(5))
            .insert_resource(CellularAutomataRngGenBool(0.55));

        #[cfg(feature = "bevy_egui")]
        app.insert_resource(CellularAutomataAutoGenerate(true))
            .add_systems(Update, ui_system);
    }
}

#[cfg(feature = "bevy_egui")]
fn ui_system(
    mut contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
    mut auto_generate: ResMut<generator::cellular_automata::CellularAutomataAutoGenerate>,
    mut iterations: ResMut<generator::cellular_automata::CellularAutomataIterations>,
    mut rng_gen_bool: ResMut<generator::cellular_automata::CellularAutomataRngGenBool>,
    mut rebuild_map_event_writer: EventWriter<RebuildMapEvent>,
) {
    let ctx = contexts.ctx_mut();

    bevy_egui::egui::Window::new("Cellular Automata Settings").show(ctx, |ui| {
        ui.add(bevy_egui::egui::Checkbox::new(
            &mut auto_generate.0,
            "auto generate",
        ));
        let iterations_slider =
            ui.add(bevy_egui::egui::Slider::new(&mut iterations.0, 0..=10).text("iterations"));
        let rng_gen_bool_slider = ui
            .add(bevy_egui::egui::Slider::new(&mut rng_gen_bool.0, 0.0..=1.0).text("rng_gen_bool"));

        let maybe_button = if **auto_generate {
            None
        } else {
            Some(ui.button("Generate"))
        };

        if (maybe_button.is_some() && maybe_button.unwrap().clicked())
            || (**auto_generate && (iterations_slider.changed() || rng_gen_bool_slider.changed()))
        {
            rebuild_map_event_writer.send(RebuildMapEvent);
        }
    });
}

#[derive(Resource, Deref, DerefMut, PartialEq, Eq, Default)]
pub struct CellularAutomataIterations(pub usize);

#[derive(Resource, Deref, DerefMut, PartialEq, Default)]
pub struct CellularAutomataRngGenBool(pub f64);

#[derive(Resource, Deref, DerefMut, PartialEq, Eq, Default)]
#[cfg(feature = "bevy_egui")]
pub struct CellularAutomataAutoGenerate(pub bool);

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
