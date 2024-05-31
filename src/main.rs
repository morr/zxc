use zxc::*;

fn main() {
    apply_global_config(load_config());

    App::new()
        // .insert_resource(CONFIG.get().unwrap().clone())
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Automatic,
                        mode: bevy::window::WindowMode::Windowed,
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        resolution: (
                            config().app.resolution.0 as f32,
                            config().app.resolution.1 as f32,
                        )
                            .into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    update_subscriber: Some(|_| configure_logging("info,zxc=trace")),
                    // update_subscriber: Some(|_| configure_logging("info,zxc=trace")),
                    ..default()
                }),
        )
        // .add_plugins(bevy_inspector_egui::quick::FilterQueryInspectorPlugin::<With<Commandable>>::default())
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
        ))
        .add_plugins((
            pawn::PawnPlugin,
            restable::RestablePlugin,
            workable::WorkablePlugin,
            movable::MovablePlugin,
            commandable::CommandablePlugin,
        ))
        .add_systems(FixedUpdate, bevy::window::close_on_esc)
        .run();
}
