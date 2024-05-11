use super::*;

pub fn track_time(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut elapsed_time: ResMut<ElapsedTime>,
    mut event_writer: EventWriter<NewDayEvent>,
) {
    let prev_day= elapsed_time.total_days();
    elapsed_time.0 += time_scale.scale_to_seconds(time.delta_seconds());
    let new_day = elapsed_time.total_days();

    if new_day != prev_day {
        let event = NewDayEvent(new_day);
        // println!("{:?}", event);
        event_writer.send(event);
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
    if let SimulationState::Running = time_state.get() {
        if !time_scale.decrease() {
            next_state.set(SimulationState::Paused);
        }
    }
}
