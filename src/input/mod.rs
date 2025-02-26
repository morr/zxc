use bevy::window::PrimaryWindow;

use crate::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveredGridTile>()
            .add_event::<HoverEvent>()
            .add_event::<ClickEventStage0>()
            .add_event::<ClickEventStage1>();

        #[cfg(feature = "bevy_egui")]
        {
            app.init_resource::<EguiWantsFocus>()
                .add_systems(PostUpdate, check_egui_wants_focus)
                .add_systems(
                    Update,
                    mouse_input
                        .run_if(in_state(AppState::Playing))
                        .run_if(resource_equals(EguiWantsFocus(false))),
                );
        }

        #[cfg(not(feature = "bevy_egui"))]
        app.add_systems(Update, mouse_input.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct HoveredGridTile(pub Option<IVec2>);

#[derive(Component, Default)]
pub struct HoverMarker;

#[derive(Event, Debug)]
pub struct HoverEvent(pub IVec2);

#[derive(Event, Debug)]
pub struct ClickEventStage0(pub IVec2);

#[derive(Event, Debug)]
pub struct ClickEventStage1(pub IVec2);

#[derive(Resource, Deref, DerefMut, PartialEq, Eq, Default)]
#[cfg(feature = "bevy_egui")]
struct EguiWantsFocus(bool);

// https://github.com/johanhelsing/bevy_pancam/blob/main/src/lib.rs#L44C1-L58C2
#[cfg(feature = "bevy_egui")]
fn check_egui_wants_focus(
    mut contexts: Query<&mut bevy_egui::EguiContext>,
    mut wants_focus: ResMut<EguiWantsFocus>,
) {
    let ctx = contexts.iter_mut().next();
    let new_wants_focus = if let Some(ctx) = ctx {
        let ctx = ctx.into_inner().get_mut();
        ctx.wants_pointer_input() || ctx.wants_keyboard_input()
    } else {
        false
    };
    wants_focus.set_if_neq(EguiWantsFocus(new_wants_focus));
}

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
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        let grid_tile = world_position.world_pos_to_grid();
        if !Navmesh::is_in_range(grid_tile.x, grid_tile.y) {
            return;
        };

        let event = HoverEvent(grid_tile);
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
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            let event = ClickEventStage0(world_position.world_pos_to_grid());
            // println!("{:?}", event);
            click_event_writer.send(log_event!(event));
        }
    }
}
