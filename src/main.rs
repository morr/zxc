pub mod configs;
use bevy::sprite::MaterialMesh2dBundle;
pub use configs::*;

mod map;
mod settings;

mod pawn;
use pawn::*;

mod structure;
use structure::*;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    prelude::*,
    window::{close_on_esc, PresentMode},
};
use bevy_pancam::{PanCam, PanCamPlugin};

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
        .add_plugins(PanCamPlugin::default())
        // .add_plugins(LogDiagnosticsPlugin::default())
        // .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(pawn::PawnPlugin)
        .add_plugins(map::MapPlugin)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        // .insert_resource(ClearColor(Color::rgba_u8(
        //     BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
        // )))
        .add_systems(Startup, (spawn_camera, spawn_base, spawn_pawns))
        .add_systems(Update, close_on_esc)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    println!("Spawning camera");

    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
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

    commands.spawn((
        StructureBundle {
            structure: Structure {
                x: -1,
                y: -1,
                width: 1,
                height: 1,
            },
            name: Name::new("Pawn"),
        },
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            // transform: Transform::from_xyz(
            //     // Distribute shapes from -X_EXTENT to +X_EXTENT.
            //     -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
            //     0.0,
            //     0.0,
            // ),
            ..default()
        },
    ));
}

fn spawn_pawns(mut commands: Commands) {
    println!("Spawning pawns");
    commands.spawn(pawn::Pawn {});
}
