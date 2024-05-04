use std::time::Duration;

use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

#[derive(Resource, Deref, DerefMut)]
pub struct TimeScale(pub f32);
impl Default for TimeScale {
    fn default() -> Self {
        Self(1.0)
    }
}

/// NOTE: simulation is not working properly on time scale >~ 75
impl TimeScale {
    pub fn scale_to_seconds(&self, seconds: f32) -> f32 {
        seconds * self.0
    }

    pub fn scale_to_duration(&self, seconds: f32) -> Duration {
        Duration::from_secs_f32(seconds * self.0)
    }

    pub fn increase(&mut self) {
        self.0 += if self.0 < 5.0 {
            2.0
        } else if self.0 < 15.0 {
            5.0
        } else if self.0 < 20.0 {
            10.0
        } else if self.0 < 100.0 {
            25.0
        } else if self.0 < 200.0 {
            50.0
        } else {
            100.0
        }
    }

    pub fn decrease(&mut self) -> bool {
        if self.0 == 1.0 {
            return false;
        }

        self.0 -= if self.0 <= 5.0 {
            2.0
        } else if self.0 <= 15.0 {
            5.0
        } else if self.0 <= 20.0 {
            10.0
        } else if self.0 <= 100.0 {
            25.0
        } else if self.0 <= 200.0 {
            50.0
        } else {
            100.0
        };

        true
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct ElapsedTime(pub f32);

impl Default for ElapsedTime {
    fn default() -> Self {
        Self(CONFIG.time.hour_duration * CONFIG.starting_scene.day_hour as f32)
    }
}

impl ElapsedTime {
    pub fn total_seconds(&self) -> f32 {
        self.0.floor()
    }

    pub fn game_time_of_day(&self) -> f32 {
        (self.0 % CONFIG.time.day_duration) / CONFIG.time.day_duration
    }

    pub fn game_day(&self) -> f32 {
        (self.0 / CONFIG.time.day_duration).floor()
    }

    pub fn game_hours(&self) -> f32 {
        ((self.0 % CONFIG.time.day_duration) / CONFIG.time.hour_duration).floor()
    }

    pub fn game_minutes(&self) -> f32 {
        (((self.0 % CONFIG.time.day_duration) % CONFIG.time.hour_duration)
            / CONFIG.time.minute_duration)
            .floor()
    }
}
