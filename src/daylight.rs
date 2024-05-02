use crate::*;

pub struct DaylightPlugin;

impl Plugin for DaylightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), setup)
            .add_systems(
                FixedUpdate,
                day_night_cycle_system.run_if(in_state(AppState::Playing)),
            );
    }
}

#[derive(Component)]
struct NightOverlay;

fn setup(mut commands: Commands) {
    let grid_size = IVec2::new(CONFIG.grid.size, CONFIG.grid.size);

    // Add an overlay sprite
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.1, 0.1, 0.3, 0.0), // Initial color, mostly transparent
                custom_size: Some(grid_size.grid_tile_edge_to_world()), // Large enough to cover the screen
                //
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, NIGHT_Z_INDEX)),

            ..default()
        })
        .insert(NightOverlay);
}

fn day_night_cycle_system(
    elapsed_time: Res<ElapsedTime>,
    mut query: Query<&mut Sprite, With<NightOverlay>>,
) {
    let theta = elapsed_time.game_time_of_day() * 2.0 * std::f32::consts::PI; // Full cycle from 0 to 2π

    let transparency = 0.5 + 0.5 * theta.cos(); // Transition the transparency
    for mut sprite in query.iter_mut() {
        sprite.color.set_a(transparency * 0.7); // Adjust transparency to simulate day/night
    }
}

// BevyMagicLight2DP
// pub struct DaylightPlugin;
//
// impl Plugin for DaylightPlugin {
//     fn build(&self, _app: &mut App) {
//         app.add_plugins(BevyMagicLight2DPlugin)
//             .register_type::<BevyMagicLight2DSettings>()
//             .register_type::<LightPassParams>()
//             .insert_resource(BevyMagicLight2DSettings {
//                 light_pass_params: LightPassParams {
//                     reservoir_size: 8,
//                     smooth_kernel_size: (3, 3),
//                     direct_light_contrib: 0.5,
//                     indirect_light_contrib: 0.5,
//                     ..default()
//                 },
//                 ..default()
//             })
//             .add_systems(OnExit(WorldState::Loading), setup_light);
//     }
// }
//
// fn setup_light(mut commands: Commands) {
//     println!("spawn daylight");
//     commands.spawn((
//         SkylightLight2D {
//             color: Color::rgb_u8(255, 244, 229),
//             intensity: 0.15,
//         },
//         Name::new("global_skylight"),
//     ));
// }
