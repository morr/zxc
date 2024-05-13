use super::*;
use std::fmt;

pub struct PlantedStateDebug<'a>(pub &'a PlantedState);

impl fmt::Debug for PlantedStateDebug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let planted_state = self.0;
        f.debug_struct("PlantedState")
            .field("growth_timer", &format_args!("{:.2}s", TimerWrapper(&planted_state.growth_timer).remaining_secs()))
            .field("tending_rest_timer", &format_args!("{:.2}s", TimerWrapper(&planted_state.tending_rest_timer).remaining_secs()))
            .field("tending_rest_started_day", &planted_state.tending_rest_started_day)
            .field("is_tending_pending_for_next_day", &planted_state.is_tending_pending_for_next_day)
            .finish()
    }
}

pub struct HarvestedStateDebug<'a>(pub &'a HarvestedState);

impl fmt::Debug for HarvestedStateDebug<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let harvested_state = self.0;
        f.debug_struct("HarvestedState")
            .field("rest_timer", &format_args!("{:.2}s", TimerWrapper(&harvested_state.rest_timer).remaining_secs()))
            .finish()
    }
}

#[derive(Debug, Clone)]
struct TimerWrapper<'a>(&'a Timer);

impl TimerWrapper<'_> {
    fn remaining_secs(&self) -> f32 {
        self.0.duration().as_secs_f32() - self.0.elapsed_secs()
    }
}
