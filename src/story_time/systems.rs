use super::*;

pub fn track_time(time: Res<Time>, time_scale: Res<TimeScale>, mut elapsed_time: ResMut<ElapsedTime>) {
    elapsed_time.0 += time.delta_seconds() * time_scale.0;
}

pub fn modify_time(
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
