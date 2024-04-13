use bevy::{
    render::texture::{ImageFilterMode, ImageSamplerDescriptor},
    window::PresentMode,
};

use zxc::*;

fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins((
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
                })
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        mag_filter: ImageFilterMode::Nearest,
                        min_filter: ImageFilterMode::Nearest,
                        ..default()
                    },
                }),
            BevyMagicLight2DPlugin,
        ))
        .insert_resource(BevyMagicLight2DSettings {
            // light_pass_params: LightPassParams {
            //     reservoir_size: 32,
            //     smooth_kernel_size: (3, 3),
            //     direct_light_contrib: 0.5,
            //     indirect_light_contrib: 0.5,
            //     ..default()
            // },
            ..default()
        })
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_plugins(bevy_inspector_egui::quick::ResourceInspectorPlugin::<BevyMagicLight2DSettings>::default())
        // .add_plugins(FilterQueryInspectorPlugin::<With<structure::Structure>>::default())
        // .add_plugins(FilterQueryInspectorPlugin::<With<Movement>>::default())
        // .add_plugins(bevy_framepace::FramepacePlugin) // this fixes bevy input lag https://github.com/bevyengine/bevy/issues/3317
        .init_state::<WorldState>()
        .add_loading_state(
            LoadingState::new(WorldState::Loading).continue_to_state(WorldState::Playing),
        )
        .add_plugins((
            camera::CameraPlugin,
            story_time::StoryTimePlugin,
            tasks_queues::TaskQueuesPlugin,
            settings::SettingsPlugin,
            assets::AssetsPlugin,
            daylight::DaylightPlugin,
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
        // .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        // .insert_resource(AmbientLight {
        //     color: Color::default(),
        //     brightness: 9.75,
        // })
        // .insert_resource(ClearColor(Color::rgba_u8(
        //     BG_COLOR.0, BG_COLOR.1, BG_COLOR.2, 0,
        // )))
        .add_systems(FixedUpdate, bevy::window::close_on_esc)
        .run();
}
