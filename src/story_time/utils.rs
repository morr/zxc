use super::*;

pub fn hours_to_seconds(hours: f32) -> f32 {
    hours * config().time.hour_duration
}

pub fn days_to_seconds(days: f32) -> f32 {
    days * config().time.day_duration
}
