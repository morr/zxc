use super::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use ::noise::Perlin;

pub struct PerlinNoisePlugin;

impl Plugin for PerlinNoisePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PerlinNoiseConfig>();

        #[cfg(feature = "bevy_egui")]
        app.add_systems(Update, ui_system);
    }
}

#[derive(Resource)]
pub struct PerlinNoiseConfig {
    pub auto_generate: bool,
    pub seed: Option<u64>,
    pub frequency: f64,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl Default for PerlinNoiseConfig {
    fn default() -> Self {
        Self {
            auto_generate: true,
            seed: None,
            frequency: 0.01,
            octaves: 4,
            lacunarity: 2.0,
            persistence: 0.5,
            offset_x: 0,
            offset_y: 0,
        }
    }
}

pub fn generate(generator_config: &Res<PerlinNoiseConfig>) -> Vec<Vec<Tile>> {
    let mut rng = match generator_config.seed {
        Some(seed) => ChaCha8Rng::seed_from_u64(seed),
        None => ChaCha8Rng::from_os_rng(),
    };
    let seed: u32 = rng.random();

    let noise = generate_noise(seed, generator_config);

    let mut grid = vec![
        vec![
            Tile {
                grid_tile: IVec2::new(0, 0),
                kind: TileKind::Grass,
                noise_value: 0.0
            };
            config().grid.size as usize
        ];
        config().grid.size as usize
    ];

    for (x, row) in grid.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            cell.grid_tile.x = navmesh_index_to_grid_tile(x);
            cell.grid_tile.y = navmesh_index_to_grid_tile(y);

            let noise_index = y * config().grid.size as usize + x;
            cell.noise_value = noise[noise_index];
        }
    }

    grid
}

pub fn generate_noise(seed: u32, generator_config: &Res<PerlinNoiseConfig>) -> Vec<f32> {
    let perlin = Perlin::new(seed);

    let width = config().grid.size as usize;
    let height = config().grid.size as usize;

    let mut data = vec![0.0f32; width * height];

    // generate noise values for each pixel in the texture
    for y in 0..height {
        for x in 0..width {
            let offseted_x = x as i32 + generator_config.offset_x;
            let offseted_y = y as i32 + generator_config.offset_y;

            // scale to the grid range and apply frequency
            let nx = offseted_x as f64 * generator_config.frequency;
            let ny = offseted_y as f64 * generator_config.frequency;

            let mut noise_value = 0.0;
            let mut amplitude = 1.0;
            let mut frequency = 1.0;

            // generate octaves of noise
            for _ in 0..generator_config.octaves {
                noise_value +=
                    ::noise::NoiseFn::get(&perlin, [nx * frequency, ny * frequency, 0.0])
                        * amplitude;
                amplitude *= generator_config.persistence;
                frequency *= generator_config.lacunarity;
            }

            // normalize to 0.0 - 1.0
            let normalized_value = ((noise_value + 1.0) / 2.0) as f32;
            let index = y * width + x;

            data[index] = normalized_value;
        }
    }

    data
}


#[cfg(feature = "bevy_egui")]
fn ui_system(
    mut egui_contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
    mut generator_config: ResMut<PerlinNoiseConfig>,
    mut rebuild_map_event_writer: EventWriter<RebuildMapEvent>,
) {
    // let ctx = egui_contexts.ctx_mut();
    //
    // bevy_egui::egui::Window::new("Cellular Automata Settings").show(ctx, |ui| {
    //     ui.add(bevy_egui::egui::Checkbox::new(
    //         &mut generator_config.auto_generate,
    //         "Auto Generate",
    //     ));
    //     let iterations_slider = ui.add(
    //         bevy_egui::egui::Slider::new(&mut generator_config.iterations, 0..=10).text("Iterations"),
    //     );
    //     let smoothing_iterations_slider = ui.add(
    //         bevy_egui::egui::Slider::new(&mut generator_config.smoothing_iterations, 0..=10)
    //             .text("Smoothing Iterations"),
    //     );
    //     let initial_alive_probability_slider = ui.add(
    //         bevy_egui::egui::Slider::new(&mut generator_config.initial_alive_probability, 0..=100)
    //             .text("Initial Alive Probability"),
    //     );
    //
    //     let generate_new_seed_button = ui.button("Generate New Seed");
    //     if generate_new_seed_button.clicked() {
    //         generator_config.seed = Some(rand::random());
    //     }
    //
    //     let maybe_button = if generator_config.auto_generate {
    //         None
    //     } else {
    //         Some(ui.button("Generate"))
    //     };
    //
    //     let is_changed = iterations_slider.changed()
    //         || smoothing_iterations_slider.changed()
    //         || initial_alive_probability_slider.changed()
    //         || generate_new_seed_button.clicked();
    //
    //     if (maybe_button.is_some() && maybe_button.unwrap().clicked())
    //         || (generator_config.auto_generate && is_changed)
    //     {
    //         rebuild_map_event_writer.send(log_event!(RebuildMapEvent {
    //             generator_kind: GeneratorKind::CellularAutomata
    //         }));
    //     }
    // });
}
