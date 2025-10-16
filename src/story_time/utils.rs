use super::*;

const EPSILON: f32 = 0.0001; // used to fix edge-case rounding issue

pub fn hours_to_seconds(hours: f32) -> f32 {
    hours * config().time.hour_duration
}

pub fn days_to_seconds(days: f32) -> f32 {
    days * config().time.day_duration
}

pub fn total_seconds(elapsed_seconds: f32) -> f32 {
    elapsed_seconds + config().time.hour_duration * config().starting_scene.day_hour as f32
}

pub fn total_days(elapsed_seconds: f32) -> u32 {
    (total_seconds(elapsed_seconds) / config().time.day_duration).floor() as u32
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

pub fn current_year_day(elapsed_seconds: f32) -> u32 {
    total_day_to_year_day(total_days(elapsed_seconds))
}

pub fn current_season_day(elapsed_seconds: f32) -> u32 {
    year_day_to_season_day(current_year_day(elapsed_seconds))
}

pub fn current_year(elapsed_seconds: f32) -> u32 {
    1 + total_days(elapsed_seconds) / config().time.days_in_year
}

pub fn current_year_season(elapsed_seconds: f32) -> YearSeason {
    year_day_to_season(current_year_day(elapsed_seconds))
}

pub fn current_day_normalized_time(elapsed_seconds: f32) -> f32 /* from 0.0 to 1.0 */ {
    (total_seconds(elapsed_seconds) % config().time.day_duration) / config().time.day_duration
}

pub fn current_day_hour(elapsed_seconds: f32) -> u32 {
    ((total_seconds(elapsed_seconds) % config().time.day_duration) / config().time.hour_duration)
        .floor() as u32
}

pub fn current_hour_minute(elapsed_seconds: f32) -> u32 {
    (((total_seconds(elapsed_seconds) % config().time.day_duration) % config().time.hour_duration)
        / config().time.minute_duration + EPSILON)
        .floor() as u32
}
// }
