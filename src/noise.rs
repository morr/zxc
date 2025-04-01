use super::*;

use ::noise::Perlin;

pub struct NoisePlugin;
impl Plugin for NoisePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NoiseSettings>()
            .add_systems(Startup, generate_noise);
    }
}

#[derive(Resource)]
pub struct NoiseSettings {
    pub seed: u32,
    pub frequency: f64,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
    pub offset_x: i32,
    pub offset_y: i32,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            // seed: rand::random(),
            seed: 1655470700,
            frequency: 0.01,
            octaves: 4,
            lacunarity: 2.0,
            persistence: 0.5,
            offset_x: 0,
            offset_y: 0,
        }
    }
}

#[derive(Resource, Default)]
pub struct NoiseData(Option<Vec<f32>>);

fn generate_noise(noise_settings: Res<NoiseSettings>, mut noise_data: ResMut<NoiseData>) {
    let perlin = Perlin::new(noise_settings.seed);

    let width = config().grid.size as usize;
    let height = config().grid.size as usize;

    let mut data = vec![0.0f32; width * height];

    // Generate noise values for each pixel in the texture
    for y in 0..height {
        for x in 0..width {
            let offseted_x = x as i32 + noise_settings.offset_x;
            let offseted_y = y as i32 + noise_settings.offset_y;

            // scale to the grid range and apply frequency
            let nx = offseted_x as f64 * noise_settings.frequency;
            let ny = offseted_y as f64 * noise_settings.frequency;

            let mut noise_value = 0.0;
            let mut amplitude = 1.0;
            let mut frequency = 1.0;

            // Generate octaves of noise
            for _ in 0..noise_settings.octaves {
                noise_value +=
                    ::noise::NoiseFn::get(&perlin, [nx * frequency, ny * frequency, 0.0])
                        * amplitude;
                amplitude *= noise_settings.persistence;
                frequency *= noise_settings.lacunarity;
            }

            // normalize to 0.0 - 1.0
            let normalized_value = ((noise_value + 1.0) / 2.0) as f32;
            let index = y * width + x;

            data[index] = normalized_value;
        }
    }

    noise_data.0 = Some(data);
}
