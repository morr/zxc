use bevy::app::AppExit;
// FPS is displayed via the Apple Metal HUD (env vars in zshrc);
// uncomment to use the bevy overlay instead
// #[cfg(feature = "debug_ui")]
// use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::remote::{RemotePlugin, http::RemoteHttpPlugin};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use zxc::commandable::Commandable;
use zxc::config::{RootConfig, apply_global_config, load_config, on_root_config_changed};
use zxc::feedable::Feedable;
use zxc::movable::Movable;
use zxc::pawn::Pawn;
use zxc::prelude::*;
use zxc::restable::Restable;
use zxc::structure::Farm;
use zxc::workable::Workable;
use zxc::{
    ai, assets, async_queue, camera, carryable, commandable, daylight, feedable, input, map,
    movable, navigation, pawn, restable, story_time, structure, tasks_queue, ui, user_selection,
    workable,
};

fn main() {
    let root_config = load_config();
    apply_global_config(root_config.clone());

    let mut app = App::new();
    app.insert_resource(root_config)
        .add_systems(
            Update,
            on_root_config_changed.run_if(resource_changed::<RootConfig>),
        )
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
                        resolution: (config().app.resolution.0, config().app.resolution.1).into(),
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
        .add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
        // .add_plugins(bevy_inspector_egui::quick::FilterQueryInspectorPlugin::<
        //     With<Commandable>,
        // >::default())
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
            ui::UiPlugin,
            // #[cfg(feature = "debug_ui")]
            // FpsOverlayPlugin {
            //     config: FpsOverlayConfig {
            //         text_config: TextFont::from_font_size(14.0),
            //         text_color: Color::srgba(1.0, 1.0, 1.0, 0.75),
            //         enabled: true,
            //         ..default()
            //     },
            // },
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
        .add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default())) // bevy_debugger_mcp
        .add_systems(Update, close_on_esc);

    // #[cfg(feature = "debug_ui")]
    // app.add_systems(PostStartup, offset_fps_overlay);

    app.run();
}

// /// The overlay has no position config; nudge its root node right so it does
// /// not overlap the items stock panel in the top-left corner.
// #[cfg(feature = "debug_ui")]
// fn offset_fps_overlay(mut query: Query<(&mut Node, &GlobalZIndex)>) {
//     use bevy::dev_tools::fps_overlay::FPS_OVERLAY_ZINDEX;
//
//     for (mut node, z_index) in query.iter_mut() {
//         if z_index.0 == FPS_OVERLAY_ZINDEX {
//             node.left = px(100.);
//             node.top = px(10.);
//         }
//     }
// }

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
    mut event_writer: MessageWriter<AppExit>,
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
        event_writer.write(AppExit::Success);
        std::process::exit(0);
    }
}
