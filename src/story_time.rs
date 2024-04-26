use super::*;

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

#[derive(Resource, Deref, DerefMut)]
pub struct ElapsedTime(pub f32);

impl Default for ElapsedTime {
    fn default() -> Self {
        Self(CONFIG.time.hour_duration * CONFIG.scene.starting_hour as f32)
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
        (((self.0 % CONFIG.time.day_duration) % CONFIG.time.hour_duration) / CONFIG.time.minute_duration).floor()
    }

}

pub struct StoryTimePlugin;

impl Plugin for StoryTimePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TimeState>()
            .init_resource::<TimeScale>()
            .init_resource::<ElapsedTime>()
            .add_systems(FixedUpdate, track_time.run_if(in_state(TimeState::Running)))
            .add_systems(Update, modify_time.run_if(in_state(WorldState::Playing)));
    }
}

fn track_time(time: Res<Time>, time_scale: Res<TimeScale>, mut elapsed_time: ResMut<ElapsedTime>) {
    elapsed_time.0 += time.delta_seconds() * time_scale.0;
}

fn modify_time(
    time_state: Res<State<TimeState>>,
    mut next_state: ResMut<NextState<TimeState>>,
    mut time_scale: ResMut<TimeScale>,
    // mut ev_update_ui: EventWriter<UpdateUiEvent>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        toggle_story_time(&time_state, &mut next_state);
        // ev_update_ui.send(UpdateUiEvent {});
    }

    if keys.just_pressed(KeyCode::Equal) {
        // println!("+");
        increase_time_scale(&time_state, &mut next_state, &mut time_scale);
        // ev_update_ui.send(UpdateUiEvent {});
    }

    if keys.just_pressed(KeyCode::Minus) {
        // println!("-");
        decrease_time_scale(&time_state, &mut next_state, &mut time_scale);
        // if decrease_time_scale(&time_state, &mut next_state, &mut time_scale) {
        //   ev_update_ui.send(UpdateUiEvent {});
        // }
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
