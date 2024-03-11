use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    window::{close_on_esc, PresentMode},
};
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};

pub mod configs;
use bevy::sprite::MaterialMesh2dBundle;
pub use configs::*;

// mod map;
mod settings;

// mod pawn;
// use pawn::*;

mod structure;
use structure::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: bevy::window::WindowMode::Windowed,
                        present_mode: PresentMode::AutoNoVsync,
                        resolution: (WW as f32, WH as f32).into(),
                        title: "Zxc".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(FilterQueryInspectorPlugin::<With<structure::Structure>>::default())
        // .add_plugins(WorldInspectorPlugin::new())
        // FilterQueryInspectorPlugin::<With<pawn::components::Pawn>>::default(),
        // .add_plugins(camera::CameraPlugin)
        .add_plugins(PanCamPlugin::default())
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        // .add_plugins(pawn::PawnPlugin)
        // .add_plugins(map::MapPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        // .insert_resource(ClearColor(Color::rgba_u8(
        //     BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
        // )))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_base)
        // .add_systems(Startup, spawn_pawns)
        .add_systems(Update, close_on_esc)
        // .add_systems(Startup, spawn_paddle)
        // .add_systems(FixedUpdate, move_paddle)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    println!("Spawning camera");

    commands
        .spawn({
            let mut camera = Camera2dBundle::default();
            camera.projection.scaling_mode = ScalingMode::FixedVertical(10.0);
            camera
        })
        .insert(PanCam {
            enabled: true,
            grab_buttons: vec![MouseButton::Left, MouseButton::Middle],
            max_scale: Some(10.0),
            max_x: None,
            max_y: None,
            min_scale: 0.01,
            min_x: None,
            min_y: None,
            zoom_to_cursor: true,
        });
}

fn spawn_base(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning base");

    let mesh = Mesh::from(Rectangle::new(1.0, 1.0));
    let material = ColorMaterial::from(Color::rgb(1., 0., 0.));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    commands.spawn((StructureBundle {
        structure: Structure {
                // x: -1,
                // y: -1,
                // width: 1,
                // height: 1,
            },
        name: Name::new("Pawn"),
        mesh_bundle: MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            //     // Distribute shapes from -X_EXTENT to +X_EXTENT.
            //     -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
            //     0.0,
            //     0.0,
            // ),
            ..default()
        },
    },));
}

// const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
// const PADDLE_SIZE: Vec2 = Vec2::new(1.20, 0.2);
// const PADDLE_SPEED: f32 = 5.0;
// const PADDLE_START_Y: f32 = 0.0;
//
// const BALLE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
//
// #[derive(Component)]
// struct Paddle;
//
// fn spawn_paddle(mut commands: Commands) {
//     commands.spawn((
//         SpriteBundle {
//             transform: Transform {
//                 translation: vec3(0., PADDLE_START_Y, 0.),
//                 ..default()
//             },
//             sprite: Sprite {
//                 color: PADDLE_COLOR,
//                 custom_size: Some(PADDLE_SIZE),
//                 ..default()
//             },
//             ..default()
//         },
//         Paddle,
//     ));
// }
//
// fn move_paddle(
//     input: Res<ButtonInput<KeyCode>>,
//     time_step: Res<Time>,
//     mut query: Query<&mut Transform, With<Paddle>>,
// ) {
//     let mut paddle_transform = query.single_mut();
//     let mut direction = 0.0;
//
//     if input.pressed(KeyCode::KeyA) {
//         direction -= 1.0;
//     }
//     if input.pressed(KeyCode::KeyD) {
//         direction += 1.0;
//     }
//     // println!("move paddle {:?}", time_step);
//
//     let new_x =
//         paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds();
//     paddle_transform.translation.x = new_x;
// }
//
// // fn spawn_pawns(mut commands: Commands) {
// //     println!("Spawning pawns");
// //     commands.spawn(pawn::Pawn {});
// // }
