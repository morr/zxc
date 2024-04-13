use super::*;
use bevy::render::camera::RenderTarget;
use bevy_pancam::PanCam;
use bevy_pancam::PanCamPlugin;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin).add_systems(
            // OnExit(WorldState::Loading),
            Startup,
            spawn_camera.after(setup_post_processing_camera),
        );
    }
}

fn spawn_camera(mut commands: Commands, camera_targets: Res<CameraTargets>) {
    // let mut occluders = vec![];
    // let occluder_entity = commands
    //     .spawn((
    //         SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0., 0., 0.))),
    //         LightOccluder2D {
    //             h_size: Vec2::new(40.0, 20.0),
    //         },
    //     ))
    //     .id();
    //
    // occluders.push(occluder_entity);
    //
    // commands
    //     .spawn(SpatialBundle::default())
    //     .insert(Name::new("occluders"))
    //     .push_children(&occluders);

    // Add lights.
    // let mut lights = vec![];
    // {
    //     let spawn_light = |cmd: &mut Commands,
    //                        x: f32,
    //                        y: f32,
    //                        name: &'static str,
    //                        light_source: OmniLightSource2D| {
    //         return cmd
    //             .spawn(Name::new(name))
    //             .insert(light_source)
    //             .insert(SpatialBundle {
    //                 transform: Transform {
    //                     translation: Vec3::new(x, y, 0.0),
    //                     ..default()
    //                 },
    //                 ..default()
    //             })
    //             .id();
    //     };
    //
    //     lights.push(spawn_light(
    //         &mut commands,
    //         -128.,
    //         -128.,
    //         "left",
    //         OmniLightSource2D {
    //             intensity: 1.0,
    //             color: Color::rgb_u8(255, 0, 0),
    //             falloff: Vec3::new(1.5, 10.0, 0.005),
    //             ..default()
    //         },
    //     ));
    //     lights.push(spawn_light(
    //         &mut commands,
    //         128.,
    //         -128.,
    //         "right",
    //         OmniLightSource2D {
    //             intensity: 1.0,
    //             color: Color::rgb_u8(0, 0, 255),
    //             falloff: Vec3::new(1.5, 10.0, 0.005),
    //             ..default()
    //         },
    //     ));
    //     lights.push(spawn_light(
    //         &mut commands,
    //         0.,
    //         128.,
    //         "rop",
    //         OmniLightSource2D {
    //             intensity: 1.0,
    //             color: Color::rgb_u8(0, 255, 0),
    //             falloff: Vec3::new(1.5, 10.0, 0.005),
    //             ..default()
    //         },
    //     ));
    // }
    // commands
    //     .spawn(SpatialBundle::default())
    //     .insert(Name::new("lights"))
    //     .push_children(&lights);

    commands.spawn((
        SkylightLight2D {
            color: Color::rgb_u8(255, 244, 229),
            intensity: 0.15,
        },
        Name::new("global_skylight"),
    ));

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
                camera: Camera {
                    hdr: true,
                    target: RenderTarget::Image(camera_targets.floor_target.clone()),
                    ..default()
                },
                ..default()
            },
            Name::new("main_camera"),
            FloorCamera,
        ))
        .insert(SpriteCamera)
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
