use super::*;

pub fn track_time(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut elapsed_time: ResMut<ElapsedTime>,
    mut event_writer: MessageWriter<NewDayMessage>,
) {
    let prev_day = elapsed_time.total_days();
    elapsed_time.0 += time_scale.scale_to_seconds(time.delta_secs());
    let new_day = elapsed_time.total_days();

    if new_day != prev_day {
        // in may pass many days in one tick under very high time scale
        for total_day in (prev_day + 1)..=new_day {
            event_writer.write(log_message!(NewDayMessage(total_day)));
        }
    }
}

pub fn modify_time(
    time_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
    mut time_scale: ResMut<TimeScale>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        toggle_story_time(&time_state, &mut next_state);
    }

    if keys.just_pressed(KeyCode::Equal) {
        increase_time_scale(&time_state, &mut next_state, &mut time_scale);
    }

    if keys.just_pressed(KeyCode::Minus) {
        decrease_time_scale(&time_state, &mut next_state, &mut time_scale);
    }
}

pub fn toggle_story_time(
    time_state: &Res<State<SimulationState>>,
    next_state: &mut ResMut<NextState<SimulationState>>,
) {
    match time_state.get() {
        SimulationState::Running => next_state.set(SimulationState::Paused),
        SimulationState::Paused => next_state.set(SimulationState::Running),
    };
}

pub fn increase_time_scale(
    time_state: &Res<State<SimulationState>>,
    next_state: &mut ResMut<NextState<SimulationState>>,
    time_scale: &mut ResMut<TimeScale>,
) {
    match time_state.get() {
        SimulationState::Running => time_scale.increase(),
        SimulationState::Paused => next_state.set(SimulationState::Running),
    };
}

pub fn decrease_time_scale(
    time_state: &Res<State<SimulationState>>,
    next_state: &mut ResMut<NextState<SimulationState>>,
    time_scale: &mut ResMut<TimeScale>,
) {
    if !time_scale.decrease()
        && let SimulationState::Running = time_state.get()
    {
        next_state.set(SimulationState::Paused);
    }
}
