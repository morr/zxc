use std::{ptr, sync::Arc};

use super::*;

#[test]
fn no_beds_spawned() {
    let mut app = App::new();
    app.add_plugins(BedPlugin);

    assert_eq!(**app.world.resource::<AvailableBeds>(), 0);
}

#[test]
fn beds_spawned() {
    // Create a mutable copy of the global config
    let mut config = (*CONFIG).clone();
    config.grid.size = 2;
    config.grid.half_size = 1;

    // Use unsafe code to replace the static CONFIG
    unsafe {
        let config_ptr = &CONFIG as *const _ as *mut Lazy<RootConfig>;
        let new_lazy = Lazy::new(|| config);
        ptr::write(config_ptr, new_lazy);
    }

    let mut app = App::new();
    app.add_plugins(BedPlugin).add_systems(Startup, setup);

    app.update();

    assert_eq!(**app.world.resource::<AvailableBeds>(), 1);
}

fn setup(
    mut commands: Commands,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut available_beds: ResMut<AvailableBeds>,
) {
    let grid_tile = IVec2::new(0, 0);

    Bed::spawn(
        grid_tile,
        &mut commands,
        Handle::<Image>::default(),
        &mut arc_navmesh.write(),
        &mut available_beds,
    );
}

// Refactor how global `CONFIG` variable is stored, initialized and used.
// Requirements:
// 1. It must be accessible from any file from any place as `CONFIG` variable or `config()` function.
// 2. It must be accessible as global resource in bevy app.
// 3. I need to be able to retrieve config in integrational tests as mutable variable and to be able to modify some of its content.
// 4. Access to `config` must not be going through some kind of locking or synchronization mechanism since `config` is used a lot, access to it must be maximally efficient.

// #[test]
// fn test_modify_config() {
//     let mut config = Arc::make_mut(&mut CONFIG.clone());
//     config.app.resolution = (1280, 720);
//     // Initialize and run the app with the modified config
// }

//
// #[test]
// fn no_beds_spawned() {
//     let mut app = App::new();
//     app.add_plugins(BedPlugin);
//
//     assert_eq!(**app.world.resource::<AvailableBeds>(), 0);
// }
//
// #[test]
// fn beds_spawned() {
//     with_config(|config| {
//         config.grid.size = 2;
//         config.grid.half_size = 1;
//     });
//
//     let mut app = App::new();
//     app.add_plugins((BedPlugin, MapPlugin))
//         .add_systems(Startup, setup);
//
//     app.update();
//
//     assert_eq!(**app.world.resource::<AvailableBeds>(), 1);
// }
//
// fn setup(
//     mut commands: Commands,
//     arc_navmesh: ResMut<ArcNavmesh>,
//     mut available_beds: ResMut<AvailableBeds>,
// ) {
//     let grid_tile = IVec2::new(0, 0);
//
//     Bed::spawn(
//         grid_tile,
//         &mut commands,
//         Handle::<Image>::default(),
//         &mut arc_navmesh.write(),
//         &mut available_beds,
//     );
// }
