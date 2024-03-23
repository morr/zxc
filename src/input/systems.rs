use bevy::{input::mouse::MouseMotion, prelude::*, window::PrimaryWindow};

use super::*;

enum VerticalMovement {
    None,
    Up,
    Down,
}

enum HorizontalMovement {
    None,
    Left,
    Right,
}

pub fn smoothened_mouse_movement(
    mut motion_evr: EventReader<MouseMotion>,
    // query to get the window (so we can read the current cursor position)
    // hovered_tile_pos: &mut ResMut<HoveredTilePos>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut event_writer: EventWriter<HoverTileEvent>,
    mut prev_hovered_tile_pos: ResMut<PrevHoveredTilePos>,
) {
    for ev in motion_evr.read() {
        println!("{:?}", ev);
        let horizontal_movement = match ev.delta.x {
            x if x < 0.0 => HorizontalMovement::Left,
            x if x > 0.0 => HorizontalMovement::Right,
            _ => HorizontalMovement::None,
        };
        let vertical_movement = match ev.delta.y {
            y if y < 0.0 => VerticalMovement::Up,
            y if y > 0.0 => VerticalMovement::Down,
            _ => VerticalMovement::None,
        };

        let (camera, camera_transform) = q_camera.single();
        let window = q_window.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let x = world_pos_to_tile(
                world_position.x
                    + match horizontal_movement {
                        HorizontalMovement::Left => -0.5,
                        HorizontalMovement::Right => 0.5,
                        HorizontalMovement::None => 0.0,
                    },
            );
            let y = world_pos_to_tile(
                world_position.y
                    + match vertical_movement {
                        VerticalMovement::Up => 0.5,
                        VerticalMovement::Down => -0.5,
                        VerticalMovement::None => 0.0,
                    },
            );
            let is_changed = match prev_hovered_tile_pos.0 {
                Some(vec) => vec.x != x || vec.y != y,
                None => true,
            };

            if is_changed {
                event_writer.send(HoverTileEvent { x, y });
                // println!("HoverTileEvent {}x{}", x, y);
                prev_hovered_tile_pos.0 = Some(UVec2::new(x, y));
            }
        }
    }
}

// pub fn mouse_movement(
//     // mut motion_evr: EventReader<MouseMotion>
//     // query to get the window (so we can read the current cursor position)
//     // hovered_tile_pos: &mut ResMut<HoveredTilePos>,
//     q_window: Query<&Window, With<PrimaryWindow>>,
//     // query to get camera transform
//     q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
//     mut event_writer: EventWriter<HoverTileEvent>,
//     mut prev_hovered_tile_pos: ResMut<PrevHoveredTilePos>,
// ) {
//     let (camera, camera_transform) = q_camera.single();
//     let window = q_window.single();
//
//     // check if the cursor is inside the window and get its position
//     // then, ask bevy to convert into world coordinates, and truncate to discard Z
//     if let Some(world_position) = window
//        .cursor_position()
//         .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
//         .map(|ray| ray.origin.truncate())
//     {
//         let x = world_pos_to_tile(world_position.x);
//         let y = world_pos_to_tile(world_position.y);
//         let is_changed = match prev_hovered_tile_pos.0 {
//             Some(vec) => vec.x != x || vec.y != y,
//             None => true,
//         };
//
//         if is_changed {
//             event_writer.send(HoverTileEvent { x, y });
//             // println!("HoverTileEvent {}x{}", x, y);
//             prev_hovered_tile_pos.0 = Some(UVec2::new(x, y));
//         }
//     }
//
//     //     for ev in motion_evr.read() {
//     //         println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
//     //     }
// }
