use core::panic;
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
        self.0 += if self.0 < 5. {
            2.
        } else if self.0 < 15. {
            5.
        } else if self.0 < 20. {
            10.
        } else if self.0 < 100. {
            25.
        } else if self.0 < 200. {
            50.
        } else if self.0 < 500. {
            100.
        } else if self.0 < 2000. {
            500.
        } else {
            1000.
        }
    }

    pub fn decrease(&mut self) -> bool {
        if self.0 == 1.0 {
            return false;
        }

        self.0 -= if self.0 <= 5. {
            2.
        } else if self.0 <= 15. {
            5.
        } else if self.0 <= 25. {
            10.
        } else if self.0 <= 100. {
            25.
        } else if self.0 <= 200. {
            50.
        } else if self.0 <= 500. {
            100.
        } else if self.0 <= 2000. {
            500.
        } else {
            1000.
        };

        true
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct ElapsedTime(pub f32);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum YearSeason {
    Spring,
    Summer,
    Fall,
    Winter,
}

impl Default for ElapsedTime {
    fn default() -> Self {
        Self(CONFIG.time.hour_duration * CONFIG.starting_scene.day_hour as f32)
    }
}

impl ElapsedTime {
    pub fn total_day_to_year_day(total_day: u32) -> u32 {
        total_day % CONFIG.time.days_in_year + 1
    }

    pub fn total_seconds(&self) -> f32 {
        self.0.floor()
    }

    pub fn day_time(&self) -> f32 {
        (self.0 % CONFIG.time.day_duration) / CONFIG.time.day_duration
    }

    pub fn total_days(&self) -> u32 {
        (self.0 / CONFIG.time.day_duration).floor() as u32
    }

    pub fn year_day(&self) -> u32 {
        Self::total_day_to_year_day(self.total_days())
    }

    pub fn season_day(&self) -> u32 {
        self.total_days() % CONFIG.time.days_in_season + 1
    }

    pub fn year(&self) -> u32 {
        1 + self.total_days() / CONFIG.time.days_in_year
    }

    pub fn year_season(&self) -> YearSeason {
        match self.season_index() {
            0 => YearSeason::Spring,
            1 => YearSeason::Summer,
            2 => YearSeason::Fall,
            3 => YearSeason::Winter,
            _ => panic!("season '{}' is out of index", self.season_index()),
        }
    }

    pub fn season_index(&self) -> u32 {
        (self.total_days() / CONFIG.time.days_in_season) % CONFIG.time.seasons_in_year
    }

    pub fn day_hour(&self) -> u32 {
        ((self.0 % CONFIG.time.day_duration) / CONFIG.time.hour_duration).floor() as u32
    }

    pub fn hour_minute(&self) -> u32 {
        (((self.0 % CONFIG.time.day_duration) % CONFIG.time.hour_duration)
            / CONFIG.time.minute_duration)
            .floor() as u32
    }
}

#[derive(Event, Debug)]
pub struct NewDayEvent(pub u32);

#[derive(Event, Debug)]
pub struct NewSeasonEvent(pub YearSeason);

#[derive(Event, Debug)]
pub struct NewYearEvent(pub u32);
