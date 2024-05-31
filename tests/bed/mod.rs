use super::*;
use crate::test_utils::with_config;

#[test]
fn no_beds_spawned() {
    let mut app = App::new();
    app.add_plugins(BedPlugin);

    assert_eq!(**app.world.resource::<AvailableBeds>(), 0);
}

#[test]
fn beds_spawned() {
      test_mode! {
        with_config(|config| {
            config.grid.size = 2;
            config.grid.half_size = 1;
        });
      }

    let mut app = App::new();
    app.add_plugins((BedPlugin, MapPlugin))
        .add_systems(Startup, setup);

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
