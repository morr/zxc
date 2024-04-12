use bevy::sprite::Material2dPlugin;

use crate::*;

pub struct DaylightPlugin;

impl Plugin for DaylightPlugin {
    fn build(&self, app: &mut App) {
        // app
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
