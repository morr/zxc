use bevy::time::TimePlugin;

use super::*;

#[test]
fn lifetime_loss_to_zero_leads_to_death_event() {
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
    let mut app = App::new();

    app.add_plugins(WorkablePlugin)
        // .add_event::<EntityStateChangeEvent<PawnState>>()
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

#[test]
fn dead_pawn_returns_task_to_tasks_queue() {
    let mut app = App::new();

    app.add_plugins(WorkablePlugin)
        .add_event::<EntityStateChangeEvent<PawnState>>()
        .add_event::<PawnDeathEvent>()
        .add_systems(Update, progress_pawn_death);

    let workable_id = app.world.spawn(Workable::default()).id();

    let original_task = Task {
        entity: workable_id,
        kind: TaskKind::FarmTending,
        grid_tile: IVec2::default(),
    };
    let pawn_id = app
        .world
        .spawn(Pawn {
            state: PawnState::Working(original_task.clone()),
            ..default()
        })
        .id();

    app.world
        .resource_mut::<Events<PawnDeathEvent>>()
        .send(PawnDeathEvent(pawn_id));

    {
        let queue = app.world.resource::<TasksQueue>();
        assert!(queue.is_empty());
    }

    app.update();

    let pawn = app.world.get::<Pawn>(pawn_id).unwrap();
    assert_eq!(pawn.state, PawnState::Dead);

    let mut queue = app.world.resource_mut::<TasksQueue>();
    assert!(!queue.is_empty());
    let task = queue.get_task().unwrap();
    assert_eq!(task, original_task);
}
