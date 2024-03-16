use bevy::{
    prelude::*,
    window::{close_on_esc, PresentMode},
};
use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

pub mod configs;
pub use configs::*;
mod pawn;
mod settings;
mod structure;
// use structure::*;
mod camera;
mod utils;

use utils::TranslationHelper;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: bevy::window::WindowMode::Windowed,
                        // present_mode: PresentMode::AutoNoVsync,
                        present_mode: PresentMode::AutoVsync,
                        resolution: (WW as f32, WH as f32).into(),
                        title: "Test App".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(FilterQueryInspectorPlugin::<With<structure::Structure>>::default())
        .add_plugins(FilterQueryInspectorPlugin::<With<pawn::Pawn>>::default())
        .add_plugins((
            camera::CameraPlugin,
            structure::StructurePlugin,
            pawn::PawnPlugin,
        ))
        .add_plugins((
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        // .insert_resource(ClearColor(Color::rgba_u8(
        //     BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
        // )))
        .add_systems(Update, close_on_esc)
        .add_systems(Update, utils::render_grid)
        // .add_systems(Startup, spawn_paddle)
        // .add_systems(FixedUpdate, move_paddle)
        .run();
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
