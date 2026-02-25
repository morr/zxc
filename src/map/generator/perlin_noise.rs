use super::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy_egui")]
use bevy_egui::egui;

pub struct PerlinNoisePlugin;

impl Plugin for PerlinNoisePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "bevy_egui")]
        app.add_systems(
            bevy_egui::EguiPrimaryContextPass,
            noise_ui_system.run_if(not(in_state(DebugNoiseState::Hidden))),
        );
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoiseDistortion {
    RawValue,
    EdgeShape,
    Distortion,
    Skewed,
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
pub fn generate(map_config: &MapGeneratorConfig) -> Vec<Vec<Tile>> {
    let mut rng = match map_config.seed {
        Some(seed) => ChaCha8Rng::seed_from_u64(seed),
        None => ChaCha8Rng::from_os_rng(),
    };

    let height_noise = generate_noise(rng.random(), &map_config.general_noise);
    let humidity_noise = generate_noise(rng.random(), &map_config.general_noise);
    let props_noise = generate_noise(rng.random(), &map_config.props_noise);

    let mut grid = vec![
        vec![
            Tile {
                grid_tile: IVec2::new(0, 0),
                kind: TileKind::Grass,
                height_noise: 0.0,
                humidity_noise: 0.0,
                props_noise: 0.0
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

            cell.height_noise = height_noise[noise_index];
            cell.humidity_noise = humidity_noise[noise_index];
            cell.props_noise = props_noise[noise_index];

            cell.kind = match cell.height_noise {
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

pub fn generate_noise(seed: u32, generator_config: &PerlinNoiseConfig) -> Vec<f32> {
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
    generator_config: &PerlinNoiseConfig,
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
fn noise_ui_system(
    mut commands: Commands,
    state: Res<State<DebugNoiseState>>,
    mut egui_contexts: bevy_inspector_egui::bevy_egui::EguiContexts,
    mut map_config: ResMut<MapGeneratorConfig>,
) {
    let ctx = egui_contexts.ctx_mut().unwrap();

    bevy_egui::egui::Window::new("Perlin Noise Settings").show(ctx, |ui| {
        let mut is_changed = false;

        let map_config = &mut *map_config;
        let noise_config = match state.get() {
            DebugNoiseState::HeightNoise | DebugNoiseState::HumidityNoise => {
                &mut map_config.general_noise
            }
            DebugNoiseState::PropsNoise => &mut map_config.props_noise,
            DebugNoiseState::Hidden => unreachable!("Should not run when Hidden"),
        };

        ui.add(bevy_egui::egui::Checkbox::new(
            &mut map_config.auto_generate,
            "Auto Generate",
        ));

        is_changed |= ui
            .add(
                egui::Slider::new(&mut noise_config.frequency, 0.001..=0.2)
                    .text("Frequency")
                    .logarithmic(true),
            )
            .changed();

        let mut octaves = noise_config.octaves as i32;
        if ui
            .add(egui::Slider::new(&mut octaves, 1..=8).text("Octaves"))
            .changed()
        {
            noise_config.octaves = octaves as usize;
            is_changed = true;
        }

        is_changed |= ui
            .add(egui::Slider::new(&mut noise_config.lacunarity, 1.0..=4.0).text("Lacunarity"))
            .changed();

        is_changed |= ui
            .add(egui::Slider::new(&mut noise_config.persistence, 0.0..=1.0).text("Persistence"))
            .changed();

        // Add ComboBox for noise distortion type
        egui::ComboBox::from_label("Noise Distortion")
            .selected_text(format!("{:?}", noise_config.distortion))
            .show_ui(ui, |ui| {
                is_changed |= ui
                    .selectable_value(
                        &mut noise_config.distortion,
                        NoiseDistortion::RawValue,
                        "Raw Value",
                    )
                    .changed();
                is_changed |= ui
                    .selectable_value(
                        &mut noise_config.distortion,
                        NoiseDistortion::EdgeShape,
                        "Edge Shape",
                    )
                    .changed();
                is_changed |= ui
                    .selectable_value(
                        &mut noise_config.distortion,
                        NoiseDistortion::Distortion,
                        "Distortion",
                    )
                    .changed();
                is_changed |= ui
                    .selectable_value(
                        &mut noise_config.distortion,
                        NoiseDistortion::Skewed,
                        "Skewed",
                    )
                    .changed();
            });

        let generate_new_seed_button = ui.button("Generate New Seed");
        if generate_new_seed_button.clicked() {
            map_config.seed = Some(rand::random());
            is_changed = true;
        }

        let maybe_button = if map_config.auto_generate {
            None
        } else {
            Some(ui.button("Generate"))
        };

        if (maybe_button.is_some() && maybe_button.unwrap().clicked())
            || (map_config.auto_generate && is_changed)
        {
            commands.trigger(log_event!(RebuildMapEvent {
                generator_kind: GeneratorKind::PerlinNoise
            }));
        }
    });
}
