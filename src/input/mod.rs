use bevy::window::PrimaryWindow;

use crate::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveredGridTile>();

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
pub struct ClickStage0Event(pub IVec2);

#[derive(Event, Debug)]
pub struct ClickStage1Event(pub IVec2);

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
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<FloorCamera>>,
    mut prev_hovered_grid_tile: ResMut<HoveredGridTile>,
) {
    let (camera, camera_transform) = q_camera
        .single()
        .expect("Camera query failed in mouse_input");
    let window = q_window
        .single()
        .expect("PrimaryWindow query failed in mouse_input");
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        let grid_tile = world_position.world_pos_to_grid();
        if !Navmesh::is_in_range(grid_tile.x, grid_tile.y) {
            return;
        };

        let is_new_hover = match prev_hovered_grid_tile.0 {
            Some(vec) => vec != grid_tile,
            None => true,
        };

        if is_new_hover {
            prev_hovered_grid_tile.0 = Some(grid_tile);
            commands.trigger(log_event!(HoverEvent(grid_tile)));
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        let (camera, camera_transform) = q_camera
            .single()
            .expect("Camera query failed in mouse_input");
        let window = q_window
            .single()
            .expect("PrimaryWindow query failed in mouse_input");
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            commands.trigger(log_event!(ClickStage0Event(
                world_position.world_pos_to_grid()
            )));
        }
    }
}
