use super::*;

#[test]
fn state_changed_to_death_by_event() {
    let mut app = App::new();

    app.add_event::<EntityStateChangeEvent<PawnState>>()
        .add_event::<PawnDeathEvent>()
        .add_systems(Update, progress_pawn_death);

    let pawn_id = app
        .world
        .spawn(Pawn {
            state: PawnState::Idle,
            ..default()
        })
        .id();

    app.world
        .resource_mut::<Events<PawnDeathEvent>>()
        .send(PawnDeathEvent(pawn_id));

    // Run systems
    app.update();

    let pawn = app.world.get::<Pawn>(pawn_id).unwrap();
    assert_eq!(pawn.state, PawnState::Dead);
}
