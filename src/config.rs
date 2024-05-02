use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

pub static CONFIG: Lazy<RootConfig> = Lazy::new(load_config);

pub const TILE_Z_INDEX: f32 = 0.0;
pub const STRUCTURE_Z_INDEX: f32 = 10.0;
pub const PAWN_Z_INDEX: f32 = 20.0;
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
    pub work_amount: WorkAmountConfig,
}

impl RootConfig {
    pub fn calculate_derived_fields(&mut self) {
        self.grid.calculate_derived_fields();
        self.time.calculate_derived_fields();
    }
}

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    pub resolution: (i32, i32),
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
    pub day_hour: i32,
    pub farm_width: i32,
    pub farm_height: i32,
}

#[derive(Deserialize, Serialize)]
pub struct TimeConfig {
    /// in game seconds
    pub day_duration: f32,

    #[serde(skip)]
    pub hour_duration: f32,
    #[serde(skip)]
    pub minute_duration: f32,
}

impl TimeConfig {
    pub fn calculate_derived_fields(&mut self) {
        self.hour_duration = self.day_duration / 24.0;
        self.minute_duration = self.hour_duration / 60.0;
    }
}

#[derive(Deserialize, Serialize)]
pub struct PawnConfig {
    pub speed: f32,
    pub work_force: f32,
}

#[derive(Deserialize, Serialize)]
pub struct WorkAmountConfig {
    /// in hours
    pub farm_tile_plant: f32,
    /// in hours
    pub farm_tile_grow: f32,
    /// in hours
    pub farm_tile_harvest: f32,
}
