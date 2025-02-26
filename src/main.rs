use bevy::app::AppExit;
use zxc::*;

fn main() {
    apply_global_config(load_config());

    App::new()
        // .insert_resource(CONFIG.get().unwrap().clone())
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Automatic,
                        mode: bevy::window::WindowMode::Windowed,
                        // present_mode: bevy::window::PresentMode::AutoNoVsync,
                        present_mode: bevy::window::PresentMode::AutoVsync,
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
                    level: bevy::log::Level::TRACE,
                    filter: "info,zxc=trace".to_string(),
                    ..default()
                }),
        )
        // .add_plugins(bevy_inspector_egui::quick::FilterQueryInspectorPlugin::<With<Commandable>>::default())
        .add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin)
        .init_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(AppState::Playing),
        )
        .add_plugins((
            daylight::DaylightPlugin,
            camera::CameraPlugin,
            story_time::StoryTimePlugin,
            async_queue::AsyncQueuePlugin,
            assets::AssetsPlugin,
            map::MapPlugin,
            navigation::NavigationPlugin,
            user_selection::UserSelectPlugin,
            structure::StructurePlugin,
            #[cfg(feature = "debug_ui")]
            // ui::UiPlugin,
            input::InputPlugin,
            tasks_queue::TasksQueuePlugin,
        ))
        .add_plugins((
            ai::AiPlugin,
            pawn::PawnPlugin,
            restable::RestablePlugin,
            feedable::FeedablePlugin,
            movable::MovablePlugin,
            commandable::CommandablePlugin,
            workable::WorkablePlugin,
            carryable::CarryablePlugin,
        ))
        .add_systems(FixedUpdate, close_on_esc)
        .run();
}

pub fn close_on_esc(
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
    pawns_query: Query<(
        Entity,
        &Pawn,
        &Movable,
        &Commandable,
        &Restable,
        &Feedable,
        // Option<&commandable_state::CommandableStateIdleTag>,
        // Option<&commandable_state::CommandableStatePendingExecutionTag>,
        // Option<&commandable_state::CommandableStateExecutingTag>,
    )>,
    farms_query: Query<(Entity, &Farm, &Workable)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut event_writer: EventWriter<AppExit>,
) {
    let mut is_quiting = false;

    for (_window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            is_quiting = true;
        }
    }

    if is_quiting {
        for (entity, pawn, movable, commandable, restable, feedable) in pawns_query.iter() {
            info!("========== Pawn {:?} ==========", entity);
            info!("{:?}", pawn);
            info!(
                "{:?} {:?} {:?} {:?}",
                movable, commandable, restable, feedable,
            );
            info!("{:?}", movable);
        }
        for (entity, farm, workable) in farms_query.iter() {
            info!("========== Farm {:?} ==========", entity);
            info!("{:?}", farm);
            info!("{:?}", workable);
        }

        next_state.set(AppState::Quiting);
        event_writer.send(AppExit::Success);
    }
}
