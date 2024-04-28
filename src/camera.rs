use super::*;
use bevy_pancam::PanCam;
use bevy_pancam::PanCamPlugin;

#[derive(Component)]
pub struct FloorCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin).add_systems(
            // OnExit(WorldState::Loading),
            Startup,
            // spawn_camera.after(setup_post_processing_camera),
            spawn_camera,
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    // , camera_targets: Res<CameraTargets>) {
    commands
        .spawn((
            Camera2dBundle {
                transform: Transform {
                    translation: Vec3::new(
                        0.0, 0.0,
                        // tile_pos_to_world((GRID_COLS as f32 / 2.0) as u32),
                        // tile_pos_to_world((GRID_ROWS as f32 / 2.0) as u32),
                        0.0,
                    ),
                    ..default()
                },
                projection: OrthographicProjection {
                    // don't forget to set `near` and `far`
                    near: -1000.0,
                    far: 1000.0,
                    // initial zoom
                    scale: 1.25,
                    // ... any other settings you want to change ...
                    ..default()
                },
                // camera: Camera {
                //     hdr: true,
                //     target: RenderTarget::Image(camera_targets.floor_target.clone()),
                //     ..default()
                // },
                ..default()
            },
            Name::new("main_camera"),
            FloorCamera,
        ))
        // .insert(SpriteCamera)
        // .spawn({
        //     let mut camera = Camera2dBundle::default();
        //     println!("{:?}", camera.projection);
        //     camera.projection.scaling_mode = ScalingMode::FixedVertical(20.0);
        //     camera
        // })
        .insert(PanCam {
            enabled: true,
            grab_buttons: vec![MouseButton::Left, MouseButton::Middle],
            max_scale: Some(20.0),
            max_x: None,
            max_y: None,
            min_scale: 0.1, // 0.5,
            min_x: None,
            min_y: None,
            zoom_to_cursor: true,
        });
}
