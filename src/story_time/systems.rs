use super::*;

// pub fn track_time(
//     time: Res<Time>,
//     mut elapsed_time: ResMut<ElapsedTime>,
//     mut event_writer: MessageWriter<NewDayMessage>,
// ) {
//     let prev_day = elapsed_time.total_days();
//     elapsed_time.0 += time_scale.scale_to_seconds(time.delta_secs());
//     let new_day = elapsed_time.total_days();
//
//     if new_day != prev_day {
//         // in may pass many days in one tick under very high time scale
//         for total_day in (prev_day + 1)..=new_day {
//             event_writer.write(log_message!(NewDayMessage(total_day)));
//         }
//     }
// }

pub fn modify_time(mut time: ResMut<Time<Virtual>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        toggle_story_time(&mut time);
    }

    if keys.just_pressed(KeyCode::Equal) {
        increase_time_scale(&mut time);
    }

    if keys.just_pressed(KeyCode::Minus) {
        decrease_time_scale(&mut time);
    }
}

pub fn toggle_story_time(time: &mut Time<Virtual>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}

pub fn increase_time_scale(time: &mut Time<Virtual>) {
    if time.is_paused() {
        time.unpause();
    } else {
        let speed = time.relative_speed();
        time.set_relative_speed(speed + if speed < 5. {
            2.
        } else if speed < 15. {
            5.
        } else if speed < 20. {
            10.
        } else if speed < 100. {
            25.
        } else if speed < 200. {
            50.
        } else if speed < 500. {
            100.
        } else if speed < 2000. {
            500.
        } else {
            1000.
        });
    }
}

pub fn decrease_time_scale(time: &mut Time<Virtual>) {
    let speed = time.relative_speed();
    if time.is_paused() || speed == 1.0 {
        return;
    }

    time.set_relative_speed(speed - if speed <= 5. {
        2.
    } else if speed <= 15. {
        5.
    } else if speed <= 25. {
        10.
    } else if speed <= 100. {
        25.
    } else if speed <= 200. {
        50.
    } else if speed <= 500. {
        100.
    } else if speed <= 2000. {
        500.
    } else {
        1000.
    });
}
