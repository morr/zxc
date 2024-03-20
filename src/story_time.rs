use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum TimeState {
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

#[derive(Resource, Deref, DerefMut, Default)]
pub struct ElapsedTime(pub f32);

pub struct StoryTimePlugin;

impl Plugin for StoryTimePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TimeState>()
            .init_resource::<TimeScale>()
            .init_resource::<ElapsedTime>()
            .add_systems(FixedUpdate, track_time.run_if(in_state(TimeState::Running)));
    }
}

fn track_time(time: Res<Time>, time_scale: Res<TimeScale>, mut elapsed_time: ResMut<ElapsedTime>) {
    elapsed_time.0 += time.delta_seconds() * time_scale.0;
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

pub fn increase_time_scale(
    time_state: &Res<State<TimeState>>,
    next_state: &mut ResMut<NextState<TimeState>>,
    time_scale: &mut ResMut<TimeScale>,
) {
    match time_state.get() {
        TimeState::Running => time_scale.0 += 1.0,
        TimeState::Paused => next_state.set(TimeState::Running),
    };
}

pub fn decrease_time_scale(
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
