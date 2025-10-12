mod apply_default_global_config;

mod pawn {
    use std::time::Duration;

    use bevy::{state::app::StatesPlugin, time::TimePlugin};
    use zxc::*;

    #[test]
    fn lifetime_loss_to_zero_leads_to_death_event() {
        let mut app = App::new();

        app.add_plugins((TimePlugin, StatesPlugin, StoryTimePlugin))
            .add_message::<PawnDeathMessage>()
            .add_systems(Update, progress_pawn_dying);

        let pawn_id = app
            .world_mut()
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
        // second update needs so some time passes
        // advance time by 5 seconds
        app.world_mut()
            .resource_mut::<Time<Virtual>>()
            .advance_by(Duration::from_secs(5));
        app.update();

        // lifetime is changed
        let pawn = app.world().get::<Pawn>(pawn_id).unwrap();
        assert_eq!(pawn.lifetime, 0.0);

        // Dying component is removed
        let maybe_dying = app.world().get::<DyingMarker>(pawn_id);
        assert!(maybe_dying.is_none());

        let mut reader = app
            .world_mut()
            .resource_mut::<Messages<PawnDeathMessage>>()
            .get_cursor();
        let maybe_event = reader
            .read(app.world_mut().resource::<Messages<PawnDeathMessage>>())
            .next();

        // PawnDeathEvent is sent
        assert!(maybe_event.is_some());
        assert_eq!(maybe_event.unwrap().entity, pawn_id);
        assert_eq!(maybe_event.unwrap().reason, PawnDeathReason::OldAge);
    }

    #[test]
    fn state_changed_to_death_by_event() {
        let mut app = App::new();

        app.add_plugins((BedPlugin, WorkablePlugin, RestablePlugin, CommandablePlugin))
            // .add_message::<EntityStateChangeMessage<PawnState>>()
            .add_message::<PawnDeathMessage>()
            .add_message::<EntityStateChangeMessage<PawnState>>()
            .add_systems(Update, progress_pawn_death);

        let pawn_id = app
            .world_mut()
            .spawn((Pawn::default(), Commandable::default(), Restable::default()))
            .id();

        app.world_mut()
            .resource_mut::<Messages<PawnDeathMessage>>()
            .write(PawnDeathMessage {
                entity: pawn_id,
                reason: PawnDeathReason::OldAge,
            });

        app.update();

        let pawn = app.world().get::<Pawn>(pawn_id).unwrap();
        assert_eq!(pawn.state, PawnState::Dead);
    }
}
