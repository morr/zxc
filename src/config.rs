use bevy::math::f32;
pub use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[cfg(test)]
pub static CONFIG: Lazy<std::sync::RwLock<RootConfig>> = Lazy::new(|| std::sync::RwLock::new(load_config()));

#[cfg(not(test))]
pub static CONFIG: Lazy<RootConfig> = Lazy::new(load_config);

#[cfg(not(test))]
#[inline]
pub fn get_config() -> &'static RootConfig {
    &CONFIG
}

#[cfg(test)]
pub fn get_config() -> std::sync::RwLockReadGuard<'static, RootConfig> {
    CONFIG.read().unwrap()
}

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

#[derive(Deserialize, Serialize)]
pub struct RootConfig {
    pub app: AppConfig,
    pub grid: GridConfig,
    pub tile: TileConfig,
    pub starting_scene: StartingSceneConfig,
    pub time: TimeConfig,
    pub pawn: PawnConfig,
    pub farming: FarmingConfig,
    pub movement_cost: MovementCostConfig,
    pub stamina_cost: StaminaCostConfig,
}

impl RootConfig {
    pub fn calculate_derived_fields(&mut self) {
        self.grid.calculate_derived_fields();
        self.time.calculate_derived_fields();
        self.stamina_cost.calculate_derived_fields(&self.time);
    }
}

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    pub resolution: (u32, u32),
}

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct TileConfig {
    /// size in pixels
    pub size: f32,
}

#[derive(Deserialize, Serialize)]
pub struct StartingSceneConfig {
    pub pawns: i32,
    pub day_hour: u32,
    pub farm_width: i32,
    pub farm_height: i32,
    pub beds_num: i32,
    pub time_scale: f32,
}

const SEASONS_IN_YEAR: u32 = 4;

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct PawnConfig {
    pub speed: f32,
    pub work_force: f32,
    pub spawn_age: (u32, u32),
    pub lifetime_span: (u32, u32),
    pub wander_when_idle: bool,
}

#[derive(Deserialize, Serialize)]
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

#[derive(Deserialize, Serialize)]
pub struct MovementCostConfig {
    /// percentage of speed reduction in number between 0.0..=1.0
    /// where 0.0 - impassable, 0.5 - half of normal speed, 1.0 0 normal speed
    pub farm: f32,
    /// percentage of speed reduction in number between 0.0..=1.0
    /// where 0.0 - impassable, 0.5 - half of normal speed, 1.0 0 normal speed
    pub furniture: f32,
}

#[derive(Deserialize, Serialize)]
pub struct StaminaCostConfig {
    // /// amount of stamina change per in-game hour of idling
    // pub idle: f32,
    // /// amount of stamina change per in-game hour of moving
    // pub moving: f32,
    // /// amount of stamina change per in-game hour of working
    // pub working: f32,
    /// amount of stamina change per in-game hour of sleeping
    pub sleeping: f32,
    /// amount of stamina change per in-game hour of living
    pub living: f32,
}

impl StaminaCostConfig {
    pub fn calculate_derived_fields(&mut self, time: &TimeConfig) {
        // convert desired numbers to proper values according to ingame hour duration
        // self.idle /= time.hour_duration;
        // self.moving /= time.hour_duration;
        // self.working /= time.hour_duration;
        self.sleeping /= time.hour_duration;
        self.living /= time.hour_duration;
    }
}
