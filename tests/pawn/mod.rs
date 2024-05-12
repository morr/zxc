use bevy::time::TimePlugin;

use super::*;

#[test]
fn lifetime_loss_to_zero_leads_to_death_event() {
    let mut app = App::new();

    app.add_plugins((TimePlugin, StoryTimePlugin))
        .add_event::<EntityStateChangeEvent<PawnState>>()
        .add_event::<PawnDeathEvent>()
        .add_systems(Update, progress_pawn_dying);

    let pawn_id = app
        .world
        .spawn(Pawn {
            state: PawnState::Idle,
            lifetime: 10.0,
            ..default()
        })
        .id();

    // https://github.com/bevyengine/bevy/blob/main/crates/bevy_time/src/time.rs
     let mut time = app.world.resource_mut::<Time>();
     let last_update = time.last_update().unwrap();
     time.update_with_instant(last_update + Duration::from_millis(30));

    app.update();
}

#[test]
fn state_changed_to_death_by_event() {
    let mut app = App::new();

    app.add_event::<EntityStateChangeEvent<PawnState>>()
        .add_event::<PawnDeathEvent>()
        .add_systems(Update, progress_pawn_death);

    let pawn_id = app.world.spawn(Pawn::default()).id();

    app.world
        .resource_mut::<Events<PawnDeathEvent>>()
        .send(PawnDeathEvent(pawn_id));

    app.update();

    let pawn = app.world.get::<Pawn>(pawn_id).unwrap();
    assert_eq!(pawn.state, PawnState::Dead);
}
