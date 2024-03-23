use bevy::{prelude::*, window::PrimaryWindow};

use crate::map::components::ClickTileEvent;

use super::*;

// enum VerticalMovement {
//     None,
//     Up,
//     Down,
// }
//
// enum HorizontalMovement {
//     None,
//     Left,
//     Right,
// }

pub fn mouse_input(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    // mut mouse_motion_evr: EventReader<MouseMotion>,
    // query to get the window (so we can read the current cursor position)
    // hovered_tile_pos: &mut ResMut<HoveredTilePos>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut hover_event_writer: EventWriter<HoverTileEvent>,
    mut click_event_writer: EventWriter<ClickTileEvent>,
    mut prev_hovered_tile_pos: ResMut<PrevHoveredTilePos>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let x = world_pos_to_tile(world_position.x);
        let y = world_pos_to_tile(world_position.y);

        let is_new_hover = match prev_hovered_tile_pos.0 {
            Some(vec) => vec.x != x || vec.y != y,
            None => true,
        };

        if is_new_hover {
            let event = HoverTileEvent { x, y };
            println!("{:?}", event);
            hover_event_writer.send(event);
            prev_hovered_tile_pos.0 = Some(UVec2::new(x, y));
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera.single();
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

            let event = ClickTileEvent { x, y };
            println!("{:?}", event);
            click_event_writer.send(event);
        }
    }
}

// for ev in mouse_motion_evr.read() {
//     let horizontal_movement = match ev.delta.x {
//         x if x < 0.0 => HorizontalMovement::Left,
//         x if x > 0.0 => HorizontalMovement::Right,
//         _ => HorizontalMovement::None,
//     };
//     let vertical_movement = match ev.delta.y {
//         y if y < 0.0 => VerticalMovement::Up,
//         y if y > 0.0 => VerticalMovement::Down,
//         _ => VerticalMovement::None,
//     };
//
//     let (camera, camera_transform) = q_camera.single();
//     let window = q_window.single();
//     // check if the cursor is inside the window and get its position
//     // then, ask bevy to convert into world coordinates, and truncate t discard Z
//     if let Some(world_position) = window
//         .cursor_position()
//         .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
//         .map(|ray| ray.origin.truncate())
//     {
//         println!("movement {:?}", world_position);
//         let x = world_pos_to_tile(
//             world_position.x
//                 + match horizontal_movement {
//                     HorizontalMovement::Left => -0.5,
//                     HorizontalMovement::Right => 0.5,
//                     HorizontalMovement::None => 0.0,
//                 },
//         );
//         let y = world_pos_to_tile(
//             world_position.y
//                 + match vertical_movement {
//                     VerticalMovement::Up => 0.5,
//                     VerticalMovement::Down => -0.5,
//                     VerticalMovement::None => 0.0,
//                 },
//         );
//         let is_changed = match prev_hovered_tile_pos.0 {
//             Some(vec) => vec.x != x || vec.y != y,
//             None => true,
//         };
//
//         if is_changed {
//             hover_event_writer.send(HoverTileEvent { x, y });
//             // println!("HoverTileEvent {}x{}", x, y);
//             prev_hovered_tile_pos.0 = Some(UVec2::new(x, y));
//         }
//     }
// }
