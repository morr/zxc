use bevy::{
    app::prelude::*, core_pipeline::core_2d::Camera2dBundle, ecs::system::Commands,
    input::mouse::MouseButton, math::Vec3, prelude::*, transform::components::Transform,
};
use bevy_pancam::{PanCam, PanCamPlugin};

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    // println!("Spawning camera");

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
                    ..Default::default()
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
                ..Default::default()
            },
            MainCamera,
        ))
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
