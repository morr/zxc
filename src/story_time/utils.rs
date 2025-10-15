use super::*;

pub fn hours_to_seconds(hours: f32) -> f32 {
    hours * config().time.hour_duration
}

pub fn days_to_seconds(days: f32) -> f32 {
    days * config().time.day_duration
}

pub fn starting_seconds() -> f32 {
    config().time.hour_duration * config().starting_scene.day_hour as f32
}

pub fn total_days(elapsed_seconds: f32) -> u32 {
    ((starting_seconds() + elapsed_seconds) / config().time.day_duration).floor() as u32
}

pub fn total_day_to_year_day(total_day: u32) -> u32 {
    total_day % config().time.days_in_year + 1
}

pub fn year_day_to_season_day(year_day: u32) -> u32 {
    (year_day - 1) % config().time.days_in_season + 1
}

pub fn year_day_to_season(year_day: u32) -> YearSeason {
    let season_index =
        ((year_day - 1) / config().time.days_in_season) % config().time.seasons_in_year;

    match season_index {
        0 => YearSeason::Spring,
        1 => YearSeason::Summer,
        2 => YearSeason::Fall,
        3 => YearSeason::Winter,
        _ => panic!("year_day '{}' is out of season index", year_day),
    }
}

pub fn year_day_to_season_day_label(year_day: u32) -> String {
    format!(
        "{} of {:?}",
        match year_day_to_season_day(year_day) {
            1 => "1st".to_string(),
            2 => "2nd".to_string(),
            3 => "3rd".to_string(),
            other => format!("{}th", other),
        },
        year_day_to_season(year_day),
    )
}
//
//     pub fn total_seconds(&self) -> f32 {
//         self.0.floor()
//     }
//
//     pub fn day_time(&self) -> f32 {
//         (self.0 % config().time.day_duration) / config().time.day_duration
//     }
//
//     pub fn total_days(&self) -> u32 {
//         (self.0 / config().time.day_duration).floor() as u32
//     }
//
//     pub fn year_day(&self) -> u32 {
//         Self::total_day_to_year_day(self.total_days())
//     }
//
//     // pub fn season_index(&self) -> u32 {
//     //     (self.total_days() / config().time.days_in_season) % config().time.seasons_in_year
//     // }
//
//     pub fn season_day(&self) -> u32 {
//         Self::year_day_to_season_day(self.year_day())
//     }
//
//     pub fn year(&self) -> u32 {
//         1 + self.total_days() / config().time.days_in_year
//     }
//
//     pub fn year_season(&self) -> YearSeason {
//         Self::year_day_to_season(self.year_day())
//     }
//
//     pub fn day_hour(&self) -> u32 {
//         ((self.0 % config().time.day_duration) / config().time.hour_duration).floor() as u32
//     }
//
//     pub fn hour_minute(&self) -> u32 {
//         (((self.0 % config().time.day_duration) % config().time.hour_duration)
//             / config().time.minute_duration)
//             .floor() as u32
//     }
// }
