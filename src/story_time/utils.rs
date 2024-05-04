use super::*;

pub fn hours_to_seconds(hours: f32) -> f32 {
    hours * CONFIG.time.hour_duration
}

pub fn days_to_seconds(days: f32) -> f32 {
    days * CONFIG.time.day_duration
}
