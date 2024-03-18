use bevy::{
    prelude::*,
    window::{close_on_esc, PresentMode},
};
// use bevy_inspector_egui::quick::FilterQueryInspectorPlugin;

mod camera;
mod pawn;
mod settings;
pub use settings::*;
mod map;
mod structure;
mod ui;
mod utils;
// use utils::TranslationHelper;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
enum TimeState {
    // MainMenu,
    #[default]
    Running,
    Paused,
}

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
        // .add_plugins(FilterQueryInspectorPlugin::<With<structure::Structure>>::default())
        // .add_plugins(FilterQueryInspectorPlugin::<With<pawn::Pawn>>::default())
        .init_state::<TimeState>()
        .add_plugins((
            camera::CameraPlugin,
            settings::SettingsPlugin,
            map::MapPlugin,
            structure::StructurePlugin,
            ui::UiPlugin,
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
        .add_systems(FixedUpdate, close_on_esc)
        // .add_systems(Startup, spawn_paddle)
        // .add_systems(FixedUpdate, move_paddle)
        .run();
}
