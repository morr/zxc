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

        // PawnPlugin registers on_pawn_entity_state_change which needs Text2dWriter,
        // so MinimalPlugins + AssetPlugin + TextPlugin are required for text infrastructure.
        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            bevy::text::TextPlugin,
            BedPlugin,
            WorkablePlugin,
            RestablePlugin,
            CommandablePlugin,
            PawnPlugin,
        ));

        let pawn_id = app
            .world_mut()
            .spawn((Pawn::default(), Commandable::default(), Restable::default()))
            .id();

        // Trigger via Commands (deferred) so it runs during the next update
        app.world_mut().commands().trigger(PawnDeathEvent {
            entity: pawn_id,
            reason: PawnDeathReason::OldAge,
        });

        app.update();

        let pawn = app.world().get::<Pawn>(pawn_id).unwrap();
        assert_eq!(pawn.state, PawnState::Dead);
    }
}
