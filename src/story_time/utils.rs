use super::*;

pub fn hours_to_seconds(hours: f32) -> f32 {
    hours * CONFIG.time.hour_duration
}
