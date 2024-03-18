use bevy::{
    app::prelude::*, core_pipeline::core_2d::Camera2dBundle, ecs::system::Commands,
    input::mouse::MouseButton,
};
use bevy_pancam::{PanCam, PanCamPlugin};

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
        .spawn(Camera2dBundle::default())
        // .spawn({
        //     let mut camera = Camera2dBundle::default();
        //     println!("{:?}", camera.projection);
        //     camera.projection.scaling_mode = ScalingMode::FixedVertical(20.0);
        //     camera
        // })
        .insert(PanCam {
            enabled: true,
            grab_buttons: vec![MouseButton::Left, MouseButton::Middle],
            max_scale: Some(5.0),
            max_x: None,
            max_y: None,
            min_scale: 0.5,
            min_x: None,
            min_y: None,
            zoom_to_cursor: true,
        });
}
