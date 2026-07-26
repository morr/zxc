use arc_swap::ArcSwapOption;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;
pub use std::sync::LazyLock;
use std::{fs::File, io::Read};

pub const TILE_Z_INDEX: f32 = 0.0;
pub const PROP_Z_INDEX: f32 = 5.0;
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

static CONFIG: ArcSwapOption<RootConfig> = ArcSwapOption::const_empty();

pub fn apply_global_config(config: RootConfig) {
    CONFIG.store(Some(Arc::new(config)));
}

/// Cheap lock-free borrow of the current config snapshot (see `RootConfig`
/// docs for the resource/static relationship).
pub fn config() -> ConfigGuard {
    ConfigGuard(CONFIG.load())
}

pub struct ConfigGuard(arc_swap::Guard<Option<Arc<RootConfig>>>);

impl Deref for ConfigGuard {
    type Target = RootConfig;

    fn deref(&self) -> &RootConfig {
        self.0.as_ref().expect("Config not initialized")
    }
}

/// Config exists in two forms:
/// - the `RootConfig` ECS resource: the source of truth, visible and editable
///   via the inspector / BRP;
/// - the `CONFIG` static (`config()`): a snapshot for code without ECS access
///   (coordinate helpers, `Default` impls, required-component constructors).
///
/// After a runtime edit of the resource, `on_root_config_changed` recalculates
/// derived fields and re-publishes the snapshot, so `config()` readers see the
/// edit too (from their next call; values captured at spawn stay as spawned).
///
/// Prefer `Res<RootConfig>` in new code.
#[derive(Resource, Reflect, Deserialize, Serialize, Clone, Debug)]
#[reflect(Resource)]
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

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
pub struct AppConfig {
    pub resolution: (u32, u32),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States, Reflect, Deserialize, Serialize)]
pub enum DebugNoiseState {
    #[default]
    Hidden,
    HeightNoise,
    HumidityNoise,
    PropsNoise,
}

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
pub struct DebugConfig {
    pub is_grid: bool,
    pub is_navmesh: bool,
    pub noise_state: DebugNoiseState,
}

#[derive(Resource, Reflect, Deserialize, Serialize, Clone, Debug)]
#[reflect(Resource)]
pub struct MapGeneratorConfig {
    pub auto_generate: bool,
    pub seed: Option<u64>,
    pub general_noise: PerlinNoiseConfig,
    pub props_noise: PerlinNoiseConfig,
}

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
pub struct PerlinNoiseConfig {
    pub frequency: f64,
    pub octaves: usize,
    pub lacunarity: f64,
    pub persistence: f64,
    pub offset_x: i32,
    pub offset_y: i32,
    pub distortion: crate::map::generator::perlin_noise::NoiseDistortion,
}

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
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

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
pub struct TileConfig {
    /// size in pixels
    pub size: f32,
}

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
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

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
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

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
pub struct PawnConfig {
    pub speed: f32,
    pub work_force: f32,
    pub spawn_age: (u32, u32),
    pub lifetime_span: (u32, u32),
    pub wander_when_idle: bool,
}

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
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

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
pub struct MovementCostConfig {
    /// percentage of speed reduction in number between 0.0..=1.0
    /// where 0.0 - impassable, 0.5 - half of normal speed, 1.0 0 normal speed
    pub farm: f32,
    /// percentage of speed reduction in number between 0.0..=1.0
    /// where 0.0 - impassable, 0.5 - half of normal speed, 1.0 0 normal speed
    pub furniture: f32,
}

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
pub struct RestableConfig {
    pub resting_on_ground_multiplier: f32,
    pub resting_on_bed_multiplier: f32,
    /// amount of stamina change per in-game hour of sleeping
    pub resting_cost: f32,
    /// amount of stamina change per in-game hour of living
    pub activity_cost: f32,

    /// derived: stamina change per virtual second (not in RON)
    #[serde(skip)]
    pub resting_cost_per_second: f32,
    /// derived: stamina change per virtual second (not in RON)
    #[serde(skip)]
    pub activity_cost_per_second: f32,
}

impl RestableConfig {
    // derived fields must stay a pure function of the RON fields so the
    // recalculation after a runtime config edit is idempotent
    pub fn calculate_derived_fields(&mut self, time: &TimeConfig) {
        self.resting_cost_per_second = self.resting_cost / time.hour_duration;
        self.activity_cost_per_second = self.activity_cost / time.hour_duration;
    }
}

#[derive(Reflect, Deserialize, Serialize, Clone, Debug)]
pub struct FeedableConfig {
    /// how many food units a pawn consumes per in-game day
    pub food_consumption_per_day: f32,
    /// how much saturation 1 food restores
    pub hunger_per_food: f32,
    /// saturation drained per real-time second (derived, not in RON)
    #[serde(skip)]
    pub hunger_drain_per_hour: f32,
}

impl FeedableConfig {
    pub fn calculate_derived_fields(&mut self, time: &TimeConfig) {
        self.hunger_drain_per_hour =
            self.food_consumption_per_day * self.hunger_per_food / 24.0 / time.hour_duration;
    }
}

/// Recalculates derived fields after the `RootConfig` resource is edited at
/// runtime (inspector / BRP) and re-publishes the `config()` snapshot from it.
/// `bypass_change_detection` prevents the write from re-triggering the
/// `resource_changed` run condition forever.
pub fn on_root_config_changed(mut config: ResMut<RootConfig>) {
    let config = config.bypass_change_detection();
    config.calculate_derived_fields();
    apply_global_config(config.clone());
    info!("RootConfig changed, derived fields recalculated and republished");
}
