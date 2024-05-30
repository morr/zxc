use bevy::window::PrimaryWindow;

use super::*;

pub fn mouse_input(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    // mut mouse_motion_evr: EventReader<MouseMotion>,
    // query to get the window (so we can read the current cursor position)
    // hovered_tile_pos: &mut ResMut<HoverMarkerTilePos>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<FloorCamera>>,
    mut hover_event_writer: EventWriter<HoverEvent>,
    mut click_event_writer: EventWriter<ClickEventStage0>,
    mut prev_hovered_grid_tile: ResMut<HoveredGridTile>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let event = HoverEvent(world_position.world_pos_to_grid());
        // println!("{:?}", event);

        let is_new_hover = match prev_hovered_grid_tile.0 {
            Some(vec) => vec != event.0,
            None => true,
        };

        if is_new_hover {
            // println!("{:?}", event);
            prev_hovered_grid_tile.0 = Some(event.0);
            hover_event_writer.send(log_event!(event));
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
            let event = ClickEventStage0(world_position.world_pos_to_grid());
            // println!("{:?}", event);
            click_event_writer.send(log_event!(event));
        }
    }
}
