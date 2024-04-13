use crate::*;

pub struct DaylightPlugin;

impl Plugin for DaylightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BevyMagicLight2DPlugin)
            .register_type::<BevyMagicLight2DSettings>()
            .register_type::<LightPassParams>()
            .insert_resource(BevyMagicLight2DSettings {
                light_pass_params: LightPassParams {
                    reservoir_size: 8,
                    smooth_kernel_size: (3, 3),
                    direct_light_contrib: 0.5,
                    indirect_light_contrib: 0.5,
                    ..default()
                },
                ..default()
            });
        // .insert_resource(BevyMagicLight2DSettings {
        //     // light_pass_params: LightPassParams {
        //     //     reservoir_size: 32,
        //     //     smooth_kernel_size: (3, 3),
        //     //     direct_light_contrib: 0.5,
        //     //     indirect_light_contrib: 0.5,
        //     //     ..default()
        //     // },
        //     ..default()
        // })
        //     .add_plugin(Material2dPlugin::<StandardMaterial2d>::default())
        //     .add_systems(OnExit(WorldState::Loading), setup_light);
    }
}

// fn setup_light(mut commands: Commands) {
//     println!("daylight");
//     commands.spawn(DirectionalLightBundle {
//         directional_light: DirectionalLight {
//             illuminance: 50000.0, // You can adjust this value for initial brightness
//             ..default()
//         },
//         ..default()
//     });
//     // Add other setup code here, e.g., Camera, entities
// }
