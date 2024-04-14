use crate::*;

pub struct DaylightPlugin;

impl Plugin for DaylightPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_plugins(BevyMagicLight2DPlugin)
        //     .register_type::<BevyMagicLight2DSettings>()
        //     .register_type::<LightPassParams>()
        //     .insert_resource(BevyMagicLight2DSettings {
        //         light_pass_params: LightPassParams {
        //             reservoir_size: 8,
        //             smooth_kernel_size: (3, 3),
        //             direct_light_contrib: 0.5,
        //             indirect_light_contrib: 0.5,
        //             ..default()
        //         },
        //         ..default()
        //     })
        //     .add_systems(OnExit(WorldState::Loading), setup_light);
    }
}

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
