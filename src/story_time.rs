use std::ops::AddAssign;

use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum TimeState {
    // MainMenu,
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

// impl AddAssign for TimeScale {
//     fn add_assign(&mut self, other: f32) {
//         *self = Self(self.0 + other);
//     }
// }

pub struct StoryTimePlugin;

impl Plugin for StoryTimePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TimeState>().init_resource::<TimeScale>();
    }
}

pub fn toggle_story_time(
    time_state: &Res<State<TimeState>>,
    next_state: &mut ResMut<NextState<TimeState>>,
) {
    match time_state.get() {
        TimeState::Running => next_state.set(TimeState::Paused),
        TimeState::Paused => next_state.set(TimeState::Running),
    };
}

pub fn increase_story_time(
    time_state: &Res<State<TimeState>>,
    next_state: &mut ResMut<NextState<TimeState>>,
    time_scale: &mut ResMut<TimeScale>,
) {
    match time_state.get() {
        TimeState::Running => time_scale.0 += 1.0,
        TimeState::Paused => next_state.set(TimeState::Running),
    };
}

pub fn decrease_story_time(
    time_state: &Res<State<TimeState>>,
    next_state: &mut ResMut<NextState<TimeState>>,
    time_scale: &mut ResMut<TimeScale>,
) -> bool {
    if let TimeState::Running = time_state.get() {
        if time_scale.0 == 1.0 {
            next_state.set(TimeState::Paused);
        } else {
            time_scale.0 -= 1.0;
        }
        true
    } else {
        false
    }
}
