use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

pub static CONFIG: Lazy<RootConfig> = Lazy::new(load_config);

pub const TILE_Z_INDEX: f32 = 0.0;
pub const STRUCTURE_Z_INDEX: f32 = 10.0;
pub const PAWN_Z_INDEX: f32 = 20.0;
pub const NIGHT_Z_INDEX: f32 = 100.0;

pub const DAY_DURATION: f32 = 60.0; // Duration of a full day cycle in seconds
pub const HOUR_DURATION: f32 = DAY_DURATION / 24.0;
pub const MINUTE_DURATION: f32 = HOUR_DURATION / 60.0;
pub const GAME_START_TIME: f32 = 10.0; // 10AM

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
    pub scene: SceneConfig,
    pub pawn: PawnConfig,
}

impl RootConfig {
    pub fn calculate_derived_fields(&mut self) {
        self.grid.calculate_derived_fields();
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
pub struct SceneConfig {
    pub starting_pawns: i32,
}

#[derive(Deserialize, Serialize)]
pub struct PawnConfig {
    pub speed: f32,
}
