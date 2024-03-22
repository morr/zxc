use bevy::{prelude::*, window::PrimaryWindow};

use super::*;

// fn track_mouse_movement(mut motion_evr: EventReader<MouseMotion>) {
//     for ev in motion_evr.read() {
//         println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
//     }
// }

pub fn mouse_movement(
    // mut mycoords: ResMut<MyWorldCoords>,
    // query to get the window (so we can read the current cursor position)
    // hovered_tile_pos: &mut ResMut<HoveredTilePos>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut event_writer: EventWriter<HoverTileEvent>,
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
        let x = world_pos_to_tile(world_position.x);
        let y = world_pos_to_tile(world_position.y);
        event_writer.send(HoverTileEvent { x, y });

        // mycoords.0 = world_position;
        println!("HoverTileEvent {}x{}", x, y);
    }
}
