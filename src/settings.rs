use super::*;

pub const WW: i32 = 1600;
pub const WH: i32 = 900;

pub const GRID_SIZE: i32 = 500;
pub const GRID_SIZE_HALF: i32 = (GRID_SIZE as f32 / 2.0) as i32;

pub const TILE_SIZE: f32 = 32.;

pub const STARTING_PAWNS: u32 = 1;
pub const PAWN_SPEED: f32 = TILE_SIZE * 3.0;

pub const TILE_Z_INDEX: f32 = 0.0;
pub const STRUCTURE_Z_INDEX: f32 = 10.0;
pub const PAWN_Z_INDEX: f32 = 20.0;
pub const NIGHT_Z_INDEX: f32 = 100.0;

pub const DAY_DURATION: f32 = 60.0; // Duration of a full day cycle in seconds
pub const HOUR_DURATION: f32 = DAY_DURATION / 24.0;
pub const MINUTE_DURATION: f32 = HOUR_DURATION / 60.0;
pub const GAME_START_TIME: f32 = 10.0; // 10AM

#[derive(Resource, Default)]
pub struct Settings {
    // pub time_scale: f32,
}

// impl Default for Settings {
//     fn default() -> Self {
//         Self { time_scale: 1.0 }
//     }
// }

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Settings>()
            .add_systems(Update, update_settings);
    }
}

fn update_settings(
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
        println!("+");
        increase_time_scale(&time_state, &mut next_state, &mut time_scale);
        // ev_update_ui.send(UpdateUiEvent {});
    }

    if keys.just_pressed(KeyCode::Minus) {
        println!("-");
        decrease_time_scale(&time_state, &mut next_state, &mut time_scale);
        // if decrease_time_scale(&time_state, &mut next_state, &mut time_scale) {
        //   ev_update_ui.send(UpdateUiEvent {});
        // }
    }
}
