use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

#[derive(Deserialize, Serialize)]
pub struct RootConfig {
    pub app: AppConfig,
    pub scene: SceneConfig,
}

impl RootConfig {
    pub fn calculate_derived_fields(&mut self) {
        self.scene.calculate_derived_fields();
    }
}

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    pub resolution: (i32, i32),
}

impl AppConfig {
    pub fn window_resolution(&self) -> bevy::window::WindowResolution {
        (self.resolution.0 as f32, self.resolution.1 as f32).into()
    }
}

#[derive(Deserialize, Serialize)]
pub struct SceneConfig {
    pub grid_size: i32,
    pub starting_pawns: i32,
    pub pawn_speed: f32,

    #[serde(skip)]
    pub grid_size_half: i32,
}

impl SceneConfig {
    pub fn calculate_derived_fields(&mut self) {
        // self.grid_size_half = (self.grid_size as f32 / 2.0) as i32;
        self.grid_size_half = self.grid_size / 2;
    }
}

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

pub static C: Lazy<RootConfig> = Lazy::new(load_config);

pub const GRID_SIZE: i32 = 500;
pub const GRID_SIZE_HALF: i32 = (GRID_SIZE as f32 / 2.0) as i32;

pub const TILE_SIZE: f32 = 32.;

pub const STARTING_PAWNS: u32 = 6;
pub const PAWN_SPEED: f32 = TILE_SIZE * 3.0;

pub const TILE_Z_INDEX: f32 = 0.0;
pub const STRUCTURE_Z_INDEX: f32 = 10.0;
pub const PAWN_Z_INDEX: f32 = 20.0;
pub const NIGHT_Z_INDEX: f32 = 100.0;

pub const DAY_DURATION: f32 = 60.0; // Duration of a full day cycle in seconds
pub const HOUR_DURATION: f32 = DAY_DURATION / 24.0;
pub const MINUTE_DURATION: f32 = HOUR_DURATION / 60.0;
pub const GAME_START_TIME: f32 = 10.0; // 10AM
