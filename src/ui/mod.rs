use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

pub mod components;
// pub use components::*;

mod debug_grid;

mod systems;
pub use systems::*;

use crate::camera::MainCamera;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(debug_grid::DebugGridPlugin)
            // .add_event::<UpdateUiEvent>()
            .add_systems(Startup, render_ui)
            .add_systems(Update, (update_ui, handle_ui_keys))
            .add_systems(Update, (track_mouse_movement, my_cursor_system));
    }
}

fn track_mouse_movement(mut motion_evr: EventReader<MouseMotion>) {
    for ev in motion_evr.read() {
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
    }
}

fn my_cursor_system(
    // mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // mycoords.0 = world_position;
        eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}
