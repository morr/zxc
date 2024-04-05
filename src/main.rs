use bevy::window::PresentMode;

use zxc::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Automatic,
                        mode: bevy::window::WindowMode::Windowed,
                        present_mode: PresentMode::AutoNoVsync,
                        // present_mode: PresentMode::AutoVsync,
                        resolution: (WW as f32, WH as f32).into(),
                        // title: "Test App".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // .add_plugins(WorldInspectorPlugin::new())
        // .add_plugins(FilterQueryInspectorPlugin::<With<structure::Structure>>::default())
        // .add_plugins(FilterQueryInspectorPlugin::<With<Movement>>::default())
        // .add_plugins(bevy_framepace::FramepacePlugin) // this fixes bevy input lag https://github.com/bevyengine/bevy/issues/3317
        .init_state::<WorldState>()
        .add_loading_state(
            LoadingState::new(WorldState::Loading).continue_to_state(WorldState::Playing),
        )
        // .add_plugins(bevy_tokio_tasks::TokioTasksPlugin {
        //     make_runtime: Box::new(|| {
        //         let mut runtime = tokio::runtime::Builder::new_multi_thread();
        //         runtime.enable_all();
        //         runtime.build().unwrap()
        //     }),
        //     ..bevy_tokio_tasks::TokioTasksPlugin::default()
        // })
        .add_plugins((
            camera::CameraPlugin,
            story_time::StoryTimePlugin,
            tasks_queues::TaskQueuesPlugin,
            settings::SettingsPlugin,
            assets::AssetsPlugin,
            map::MapPlugin,
            movement::MovementPlugin,
            navigation::NavigationPlugin,
            structure::StructurePlugin,
            ui::UiPlugin,
            input::InputPlugin,
            pawn::PawnPlugin,
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
        .run();
}
