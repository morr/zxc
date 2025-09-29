use super::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[cfg(feature = "bevy_egui")]
use bevy_egui::egui;

pub struct PerlinNoisePlugin;

impl Plugin for PerlinNoisePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PerlinNoiseConfig>();

        #[cfg(feature = "bevy_egui")]
        app.add_systems(bevy_egui::EguiPrimaryContextPass, ui_system);
    }
}

#[derive(Debug, PartialEq)]
pub enum NoiseDistortion {
    RawValue,
    EdgeShape,
    Distortion,
    Skewed,
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
    pub distortion: NoiseDistortion,
}

impl Default for PerlinNoiseConfig {
    fn default() -> Self {
        Self {
            auto_generate: true,
            seed: Some(15759246571423803862),
            frequency: 0.01,
            octaves: 4,
            lacunarity: 2.0,
            persistence: 0.5,
            offset_x: 0,
            offset_y: 0,
            distortion: NoiseDistortion::RawValue,
        }
    }
}

// http://www-cs-students.stanford.edu/~amitp/game-programming/polygon-map-generation/

// Biome types:
// high elevations get snow, rock, tundra;
// medium elevations get shrubs, deserts, forests, and grassland;
// low elevations get rain forests, grassland, and beaches.

// Elevation    Moisture Zone
// Zone 6 (wet)	5	4	3	2	1 (dry)
// 4 (high)	SNOW	TUNDRA	BARE	SCORCHED
// 3	TAIGA	SHRUBLAND	TEMPERATE DESERT
// 2	TEMPERATE RAIN FOREST	TEMPERATE DECIDUOUS FOREST	GRASSLAND	TEMPERATE DESERT
// 1 (low)	TROPICAL RAIN FOREST	TROPICAL SEASONAL FOREST	GRASSLAND	SUBTROPICAL DESERT
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

            cell.kind = match cell.noise_value {
                n if n < 0.1 => TileKind::DeepWater,
                n if n < 0.15 => TileKind::ShallowWater,
                n if n < 0.2 => TileKind::Sand,
                n if n < 0.6 => TileKind::Grass,
                _ => TileKind::Forest,
                //
                // n if n < 0.2 => TileKind::DeepWater,
                // n if n < 0.3 => TileKind::ShallowWater,
                // n if n < 0.4 => TileKind::Sand,
                // n if n < 0.6 => TileKind::Grass,
                // n if n < 0.7 => TileKind::Forest,
                // n if n < 0.8 => TileKind::RockyDirt,
                // // n if n < 0.9 => TileKind::Dirt,
                // // n if n < 0.95 => TileKind::FertileDirt,
                // _ => TileKind::Mountain,
            };
        }
    }

    grid
}

pub fn generate_noise(seed: u32, generator_config: &Res<PerlinNoiseConfig>) -> Vec<f32> {
    // let noise = Perlin::new(seed);
    let noise = noise::Simplex::new(seed);

    let width = config().grid.size as usize;
    let height = config().grid.size as usize;

    let mut data = vec![0.0f32; width * height];

    // generate noise values for each pixel in the texture
    for y in 0..height {
        for x in 0..width {
            let index = y * width + x;
            let raw_value = noise_value(x, y, &noise, generator_config);

            let normalized_value = match generator_config.distortion {
                NoiseDistortion::RawValue => raw_value,
                NoiseDistortion::EdgeShape => {
                    if (0.475..=0.525).contains(&raw_value) {
                        raw_value
                    } else {
                        0.0
                    }
                }
                NoiseDistortion::Distortion => {
                    let value2 = noise_value(x + 1, y + 1, &noise, generator_config);
                    noise_value(
                        (raw_value * width as f32).floor() as usize,
                        (value2 * height as f32).floor() as usize,
                        &noise,
                        generator_config,
                    )
                }
                NoiseDistortion::Skewed => noise_value(x, y * 2, &noise, generator_config),
            };

            data[index] = normalized_value;
        }
    }

    data
}

fn noise_value(
    x: usize,
    y: usize,
    noise: &noise::Simplex,
    generator_config: &Res<PerlinNoiseConfig>,
) -> f32 {
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
            ::noise::NoiseFn::get(&noise, [nx * frequency, ny * frequency, 0.0]) * amplitude;
        amplitude *= generator_config.persistence;
        frequency *= generator_config.lacunarity;
    }

    // normalize to 0.0 - 1.0
    ((noise_value + 1.0) / 2.0) as f32
}

#[cfg(feature = "bevy_egui")]
fn ui_system(
    mut egui_contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
    mut generator_config: ResMut<PerlinNoiseConfig>,
    mut rebuild_map_event_writer: EventWriter<RebuildMapEvent>,
) {
    let ctx = egui_contexts.ctx_mut().unwrap();

    bevy_egui::egui::Window::new("Perlin Noise Settings").show(ctx, |ui| {
        let mut is_changed = false;

        ui.add(bevy_egui::egui::Checkbox::new(
            &mut generator_config.auto_generate,
            "Auto Generate",
        ));

        is_changed |= ui
            .add(
                egui::Slider::new(&mut generator_config.frequency, 0.001..=0.1)
                    .text("Frequency")
                    .logarithmic(true),
            )
            .changed();

        let mut octaves = generator_config.octaves as i32;
        if ui
            .add(egui::Slider::new(&mut octaves, 1..=8).text("Octaves"))
            .changed()
        {
            generator_config.octaves = octaves as usize;
            is_changed = true;
        }

        is_changed |= ui
            .add(egui::Slider::new(&mut generator_config.lacunarity, 1.0..=4.0).text("Lacunarity"))
            .changed();

        is_changed |= ui
            .add(
                egui::Slider::new(&mut generator_config.persistence, 0.0..=1.0).text("Persistence"),
            )
            .changed();

        // Add ComboBox for noise distortion type
        egui::ComboBox::from_label("Noise Distortion")
            .selected_text(format!("{:?}", generator_config.distortion))
            .show_ui(ui, |ui| {
                is_changed |= ui
                    .selectable_value(
                        &mut generator_config.distortion,
                        NoiseDistortion::RawValue,
                        "Raw Value",
                    )
                    .changed();
                is_changed |= ui
                    .selectable_value(
                        &mut generator_config.distortion,
                        NoiseDistortion::EdgeShape,
                        "Edge Shape",
                    )
                    .changed();
                is_changed |= ui
                    .selectable_value(
                        &mut generator_config.distortion,
                        NoiseDistortion::Distortion,
                        "Distortion",
                    )
                    .changed();
                is_changed |= ui
                    .selectable_value(
                        &mut generator_config.distortion,
                        NoiseDistortion::Skewed,
                        "Skewed",
                    )
                    .changed();
            });

        let generate_new_seed_button = ui.button("Generate New Seed");
        if generate_new_seed_button.clicked() {
            generator_config.seed = Some(rand::random());
            is_changed = true;
        }

        let maybe_button = if generator_config.auto_generate {
            None
        } else {
            Some(ui.button("Generate"))
        };

        if (maybe_button.is_some() && maybe_button.unwrap().clicked())
            || (generator_config.auto_generate && is_changed)
        {
            rebuild_map_event_writer.write(log_event!(RebuildMapEvent {
                generator_kind: GeneratorKind::PerlinNoise
            }));
        }
    });
}
