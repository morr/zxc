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
            Camera2d,
            Transform {
                translation: Vec3::new(
                    0.0,
                    0.0,
                    // tile_pos_to_world((GRID_COLS as f32 / 2.0) as u32),
                    // tile_pos_to_world((GRID_ROWS as f32 / 2.0) as u32),
                    0.0,
                ),
                ..default()
            },
            OrthographicProjection {
                near: -1000.0,
                far: 1000.0,
                // initial zoom
                scale: 1.25,
                ..OrthographicProjection::default_2d()
            },
            Name::new("main_camera"),
            FloorCamera,
            Msaa::Off
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
            move_keys: bevy_pancam::DirectionKeys {
                up: vec![KeyCode::KeyW],
                down: vec![KeyCode::KeyS],
                left: vec![KeyCode::KeyA],
                right: vec![KeyCode::KeyD],
            },
            speed: 600.,
            max_scale: 20.0,
            max_x: f32::INFINITY,
            max_y: f32::INFINITY,
            min_scale: 0.1, // 0.5,
            min_x: f32::NEG_INFINITY,
            min_y: f32::NEG_INFINITY,
            zoom_to_cursor: true,
        });
}
