pub use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

use crate::DebugNoiseState;

pub const TILE_Z_INDEX: f32 = 0.0;
pub const STRUCTURE_Z_INDEX: f32 = 10.0;
pub const PAWN_Z_INDEX: f32 = 20.0;
pub const ITEM_Z_INDEX: f32 = 40.0;
pub const NIGHT_Z_INDEX: f32 = 100.0;

pub fn load_config() -> RootConfig {
    let mut contents = String::new();

    File::open("resources/config.ron")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();

    let mut config = ron::from_str::<RootConfig>(&contents).unwrap();
    config.calculate_derived_fields();
    config
}

pub static CONFIG: OnceCell<RootConfig> = OnceCell::new();

pub fn apply_global_config(config: RootConfig) {
    CONFIG.set(config).expect("Failed to set global config");
}

pub fn config() -> &'static RootConfig {
    CONFIG.get().expect("Config not initialized")
}

pub fn config_mut() -> &'static mut RootConfig {
    unsafe {
        // SAFETY: Caller must ensure no concurrent access
        // This is UB if multiple threads access simultaneously
        (CONFIG.get().expect("Config not initialized") as *const RootConfig as *mut RootConfig)
            .as_mut()
            .unwrap()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RootConfig {
    pub app: AppConfig,
    pub debug: DebugConfig,
    pub map_generator: MapGeneratorConfig,
    pub grid: GridConfig,
    pub tile: TileConfig,
    pub starting_scene: StartingSceneConfig,
    pub time: TimeConfig,
    pub pawn: PawnConfig,
    pub farming: FarmingConfig,
    pub movement_cost: MovementCostConfig,
    pub restable: RestableConfig,
    pub feedable: FeedableConfig,
}

impl RootConfig {
    pub fn calculate_derived_fields(&mut self) {
        self.grid.calculate_derived_fields();
        self.time.calculate_derived_fields();
        self.restable.calculate_derived_fields(&self.time);
        self.feedable.calculate_derived_fields(&self.time);
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AppConfig {
    pub resolution: (u32, u32),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DebugConfig {
    pub is_grid: bool,
    pub is_navmesh: bool,
    pub noise_state: DebugNoiseState,
}

// #[derive(Resource, Deserialize, Serialize, Clone, Debug)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MapGeneratorConfig {
    pub auto_generate: bool,
    pub seed: Option<u64>,
    pub general_noise: PerlinNoiseConfig,
    pub props_noise: PerlinNoiseConfig,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PerlinNoiseConfig {
    pub frequency: f64,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
    pub offset_x: i32,
    pub offset_y: i32,
    pub distortion: crate::generator::perlin_noise::NoiseDistortion,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GridConfig {
    pub size: i32,

    #[serde(skip)]
    pub half_size: i32,
}

impl GridConfig {
    pub fn calculate_derived_fields(&mut self) {
        self.half_size = self.size / 2;
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TileConfig {
    /// size in pixels
    pub size: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct StartingSceneConfig {
    pub is_paused: bool,
    pub time_scale: f32,
    pub day_hour: u32,
    pub pawns: i32,
    pub farms: i32,
    pub beds: i32,
    pub storages: i32,
    pub food: u32,
}

const SEASONS_IN_YEAR: u32 = 4;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TimeConfig {
    /// in game seconds
    pub day_duration: f32,

    #[serde(skip)]
    // in game seconds
    pub year_duration: f32,

    pub days_in_season: u32,

    #[serde(skip)]
    /// in game seconds
    pub hour_duration: f32,

    #[serde(skip)]
    /// in game seconds
    pub minute_duration: f32,

    #[serde(skip)]
    pub seasons_in_year: u32,

    #[serde(skip)]
    pub days_in_year: u32,
}

impl TimeConfig {
    pub fn calculate_derived_fields(&mut self) {
        self.hour_duration = self.day_duration / 24.0;
        self.minute_duration = self.hour_duration / 60.0;
        self.seasons_in_year = SEASONS_IN_YEAR;
        self.days_in_year = self.days_in_season * self.seasons_in_year;
        self.year_duration = self.day_duration * self.days_in_year as f32;
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PawnConfig {
    pub speed: f32,
    pub work_force: f32,
    pub spawn_age: (u32, u32),
    pub lifetime_span: (u32, u32),
    pub wander_when_idle: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FarmingConfig {
    /// percent of `max_yield` for 0 tended tile
    pub basic_yield_percent: f32,
    /// amount of food items per fully tended tile
    pub max_yield: f32,
    /// in hours
    pub planting_hours: f32,
    /// in hours
    pub tending_hours: f32,
    /// in hours
    pub harvesting_hours: f32,
    /// in days
    pub harvested_rest_days: f32,
    /// in days
    pub growth_days: f32,
    /// in hours
    pub tending_rest_hours: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MovementCostConfig {
    /// percentage of speed reduction in number between 0.0..=1.0
    /// where 0.0 - impassable, 0.5 - half of normal speed, 1.0 0 normal speed
    pub farm: f32,
    /// percentage of speed reduction in number between 0.0..=1.0
    /// where 0.0 - impassable, 0.5 - half of normal speed, 1.0 0 normal speed
    pub furniture: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RestableConfig {
    pub resting_on_ground_multiplier: f32,
    pub resting_on_bed_multiplier: f32,
    /// amount of stamina change per in-game hour of sleeping
    pub resting_cost: f32,
    /// amount of stamina change per in-game hour of living
    pub activity_cost: f32,
}

impl RestableConfig {
    pub fn calculate_derived_fields(&mut self, time: &TimeConfig) {
        // convert desired numbers to proper values according to ingame hour duration
        // self.idle /= time.hour_duration;
        // self.moving /= time.hour_duration;
        // self.working /= time.hour_duration;
        self.resting_cost /= time.hour_duration;
        self.activity_cost /= time.hour_duration;
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FeedableConfig {
    /// amount of saturation change per in-game hour
    pub living_cost: f32,
    /// how many starvation overflows pawn can survive
    pub max_starvation_multiplier: f32,
}

impl FeedableConfig {
    pub fn calculate_derived_fields(&mut self, time: &TimeConfig) {
        self.living_cost /= time.hour_duration;
    }
}
