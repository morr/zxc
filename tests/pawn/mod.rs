use bevy::time::TimePlugin;

use super::*;

#[test]
fn lifetime_loss_to_zero_leads_to_death_event() {
    apply_global_config(load_config());

    let mut app = App::new();

    app.add_plugins((TimePlugin, StoryTimePlugin))
        .add_event::<PawnDeathEvent>()
        .add_systems(Update, progress_pawn_dying);

    let pawn_id = app
        .world
        .spawn((
            Pawn {
                state: PawnState::Idle,
                lifetime: 0.0001,
                ..default()
            },
            DyingMarker,
        ))
        .id();

    app.update();
    // second update needs so some nanoseconds pass
    app.update();

    // lifetime is changed
    let pawn = app.world.get::<Pawn>(pawn_id).unwrap();
    assert_eq!(pawn.lifetime, 0.0);

    // Dying component is removed
    let maybe_dying = app.world.get::<DyingMarker>(pawn_id);
    assert!(maybe_dying.is_none());

    let mut reader = app
        .world
        .resource_mut::<Events<PawnDeathEvent>>()
        .get_reader();
    let maybe_event = reader
        .read(app.world.resource::<Events<PawnDeathEvent>>())
        .next();

    // PawnDeathEvent is sent
    assert!(maybe_event.is_some());
    assert_eq!(maybe_event.unwrap().0, pawn_id);
}

#[test]
fn state_changed_to_death_by_event() {
    apply_global_config(load_config());

    let mut app = App::new();

    app.add_plugins((BedPlugin, WorkablePlugin, RestablePlugin, CommandablePlugin))
        // .add_event::<EntityStateChangeEvent<PawnState>>()
        .add_event::<PawnDeathEvent>()
        .add_event::<EntityStateChangeEvent<PawnState>>()
        .add_systems(Update, progress_pawn_death);

    let pawn_id = app
        .world
        .spawn((Pawn::default(), Commandable::default(), Restable::default()))
        .id();

    app.world
        .resource_mut::<Events<PawnDeathEvent>>()
        .send(PawnDeathEvent(pawn_id));

    app.update();

    let pawn = app.world.get::<Pawn>(pawn_id).unwrap();
    assert_eq!(pawn.state, PawnState::Dead);
}
