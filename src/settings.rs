use super::*;

pub const WW: i32 = 1600;
pub const WH: i32 = 900;

pub const GRID_SIZE: i32 = 1000;

pub const GRID_SIZE_HALF: i32 = (GRID_SIZE as f32 / 2.0) as i32;

pub const TILE_SIZE: f32 = 32.;

pub const STARTING_PAWNS: u32 = 1;
pub const PAWN_SPEED: f32 = TILE_SIZE * 5.0;

pub const TILE_Z_INDEX: f32 = 0.0;
pub const STRUCTURE_Z_INDEX: f32 = 10.0;
pub const PAWN_Z_INDEX: f32 = 20.0;

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
            .add_systems(FixedUpdate, update_settings);
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
        increase_time_scale(&time_state, &mut next_state, &mut time_scale);
        // ev_update_ui.send(UpdateUiEvent {});
    }

    if keys.just_pressed(KeyCode::Minus) {
        decrease_time_scale(&time_state, &mut next_state, &mut time_scale);
        // if decrease_time_scale(&time_state, &mut next_state, &mut time_scale) {
        //   ev_update_ui.send(UpdateUiEvent {});
        // }
    }
}
