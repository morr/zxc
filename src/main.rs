use bevy::window::PresentMode;

mod prelude;
pub use prelude::*;

mod assets;
mod camera;
mod input;
mod map;
mod movement;
mod navigation;
mod pawn;
mod settings;
mod story_time;
mod structure;
mod ui;
mod utils;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // position: WindowPosition::At(IVec2::new(1000, 0)),
                        position: WindowPosition::Automatic,
                        mode: bevy::window::WindowMode::Windowed,
                        // present_mode: PresentMode::AutoNoVsync,
                        present_mode: PresentMode::AutoVsync,
                        // present_mode: PresentMode::Immediate,
                        resolution: (WW as f32, WH as f32).into(),
                        // title: "Test App".to_string(),
                        // focused: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        // .add_plugins(WorldInspectorPlugin::new())
        // .add_plugins(FilterQueryInspectorPlugin::<With<structure::Structure>>::default())
        // .add_plugins(FilterQueryInspectorPlugin::<With<pawn::Pawn>>::default())
        .add_plugins(bevy_framepace::FramepacePlugin) // this fixes bevy input lag https://github.com/bevyengine/bevy/issues/3317
        .add_plugins((
            camera::CameraPlugin,
            settings::SettingsPlugin,
            map::MapPlugin,
            navigation::NavigationPlugin,
            structure::StructurePlugin,
            ui::UiPlugin,
            input::InputPlugin,
            pawn::PawnPlugin,
            movement::MovementPlugin,
            story_time::StoryTimePlugin,
        ))
        // .add_plugins((
        //     bevy::diagnostic::LogDiagnosticsPlugin::default(),
        //     bevy::diagnostic::FrameTimeDiagnosticsPlugin::default(),
        // ))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        // .insert_resource(ClearColor(Color::rgba_u8(
        //     BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
        // )))
        .add_systems(FixedUpdate, bevy::window::close_on_esc)
        // .add_systems(Startup, spawn_paddle)
        // .add_systems(FixedUpdate, move_paddle)
        .run();
}
