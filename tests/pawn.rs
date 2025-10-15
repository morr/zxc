mod apply_default_global_config;

mod pawn {
    use std::time::Duration;

    use bevy::{state::app::StatesPlugin, time::TimePlugin};
    use zxc::*;

    #[test]
    fn lifetime_loss_to_zero_leads_to_death_event() {
        let mut app = App::new();

        app.add_plugins((TimePlugin, StatesPlugin, StoryTimePlugin))
            // .add_message::<PawnDeatEvent>()
            // .add_observer(on_pawn_death)
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
        // advance time by 5 seconds
        // app.world_mut()
        //     .resource_mut::<Time<Virtual>>()
        //     .advance_by(Duration::from_secs(50000));
        std::thread::sleep(Duration::from_secs_f32(0.1));
        app.update(); // second update needs so some time passes

        // lifetime is changed
        let pawn = app.world().get::<Pawn>(pawn_id).unwrap();
        assert_eq!(pawn.lifetime, 0.0);

        // Dying component is removed
        let maybe_dying = app.world().get::<DyingMarker>(pawn_id);
        assert!(maybe_dying.is_none());

        // // Pawn state is Dead
        // let pawn = app.world().get::<Pawn>(pawn_id).unwrap();
        // assert_eq!(pawn.state, PawnState::Dead);

        // // PawnStateDeadTag component is added
        // let has_dead_tag = app.world().get::<pawn_state::PawnStateDeadTag>(pawn_id);
        // assert!(has_dead_tag.is_some())
    }

    #[test]
    fn state_changed_to_death_by_event() {
        let mut app = App::new();

        app.add_plugins((
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

        app.world_mut().trigger(PawnDeatEvent {
            entity: pawn_id,
            reason: PawnDeathReason::OldAge,
        });

        app.update();

        let pawn = app.world().get::<Pawn>(pawn_id).unwrap();
        assert_eq!(pawn.state, PawnState::Dead);
    }
}
