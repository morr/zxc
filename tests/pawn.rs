mod apply_default_global_config;

mod pawn {
    use std::time::Duration;

    use bevy::time::{TimePlugin, TimeUpdateStrategy};
    use zxc::*;

    #[test]
    fn lifetime_loss_to_zero_leads_to_death_event() {
        let mut app = App::new();

        app.add_plugins(TimePlugin)
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs(1)))
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

        // First update: delta is 0 (time initialization)
        app.update();
        // Second update: delta is 1 second, which exceeds lifetime
        app.update();

        // lifetime is changed
        let pawn = app.world().get::<Pawn>(pawn_id).unwrap();
        assert_eq!(pawn.lifetime, 0.0);

        // Dying component is removed
        let maybe_dying = app.world().get::<DyingMarker>(pawn_id);
        assert!(maybe_dying.is_none());
    }

    #[test]
    fn state_changed_to_death_by_event() {
        let mut app = App::new();

        // Register only the observers/plugins needed, without PawnPlugin
        // (which registers on_pawn_entity_state_change that requires Text2dWriter resources)
        app.add_plugins((BedPlugin, WorkablePlugin, RestablePlugin, CommandablePlugin))
            .add_observer(on_pawn_death);

        let pawn_id = app
            .world_mut()
            .spawn((Pawn::default(), Commandable::default(), Restable::default()))
            .id();

        // First update to finalize observer registration
        app.update();

        app.world_mut().trigger(PawnDeatEvent {
            entity: pawn_id,
            reason: PawnDeathReason::OldAge,
        });

        app.update();

        let pawn = app.world().get::<Pawn>(pawn_id).unwrap();
        assert_eq!(pawn.state, PawnState::Dead);
    }
}
