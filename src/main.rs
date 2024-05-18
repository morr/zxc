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
                        resolution: (
                            CONFIG.app.resolution.0 as f32,
                            CONFIG.app.resolution.1 as f32,
                        )
                            .into(),
                        // title: "Test App".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
            // .set(ImagePlugin {
            //     default_sampler: ImageSamplerDescriptor {
            //         mag_filter: ImageFilterMode::Nearest,
            //         min_filter: ImageFilterMode::Nearest,
            //         ..default()
            //     },
            // }),
        )
        // .add_plugins(EguiPlugin)
        // .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
        // .add_systems(OnExit(AppState::Loading), inspector_ui)
        // .add_plugins(WorldInspectorPlugin::new())
        // .add_plugins(FilterQueryInspectorPlugin::<With<structure::Structure>>::default())
        // .add_plugins(FilterQueryInspectorPlugin::<With<Pawn>>::default())
        // .add_plugins(bevy_framepace::FramepacePlugin) // this fixes bevy input lag https://github.com/bevyengine/bevy/issues/3317
        .init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::Playing),
        )
        .add_plugins((
            daylight::DaylightPlugin,
            camera::CameraPlugin,
            story_time::StoryTimePlugin,
            async_queue::AsyncQueuePlugin,
            item::ItemPlugin,
            assets::AssetsPlugin,
            map::MapPlugin,
            navigation::NavigationPlugin,
            user_selection::UserSelectPlugin,
            structure::StructurePlugin,
            ui::UiPlugin,
            input::InputPlugin,
            commands::CommandsPlugin,
        ))
        .add_plugins((
            pawn::PawnPlugin,
            restable::RestablePlugin,
            workable::WorkablePlugin,
            movable::MovablePlugin,
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

// fn inspector_ui(world: &mut World) {
//     let Ok(egui_context) = world
//         .query_filtered::<&mut EguiContext, With<bevy::window::PrimaryWindow>>()
//         .get_single(world)
//     else {
//         return;
//     };
//     let mut egui_context = egui_context.clone();
//
//     egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
//         egui::ScrollArea::vertical().show(ui, |ui| {
//             // equivalent to `WorldInspectorPlugin`
//             bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);
//
//             egui::CollapsingHeader::new("Materials").show(ui, |ui| {
//                 bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
//             });
//
//             ui.heading("Entities");
//             bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
//         });
//     });
// }
