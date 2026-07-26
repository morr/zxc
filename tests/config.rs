mod apply_default_global_config;

mod config_tests {
    use zxc::*;

    /// A runtime edit of the `RootConfig` resource must recalculate derived
    /// fields and re-publish the `config()` snapshot.
    #[test]
    fn runtime_config_edit_reaches_global_snapshot() {
        let mut app = App::new();
        app.insert_resource(config().as_ref().clone()).add_systems(
            Update,
            on_root_config_changed.run_if(resource_changed::<RootConfig>),
        );
        // initial run triggered by the resource being freshly added
        app.update();

        let drain_before = config().feedable.hunger_drain_per_hour;

        app.world_mut()
            .resource_mut::<RootConfig>()
            .feedable
            .food_consumption_per_day *= 6.0;
        app.update();

        let drain_after = config().feedable.hunger_drain_per_hour;
        assert!(
            (drain_after - drain_before * 6.0).abs() < 1e-4,
            "derived field must be recalculated and visible through config(): before={drain_before}, after={drain_after}"
        );

        // repeated recalculation must not drift non-edited derived fields
        let resting_before = config().restable.resting_cost_per_second;
        app.world_mut()
            .resource_mut::<RootConfig>()
            .feedable
            .food_consumption_per_day *= 1.0;
        app.update();
        assert_eq!(resting_before, config().restable.resting_cost_per_second);
    }
}

#[cfg(test)]
mod bench {
    use super::config_tests as _;
    use std::sync::OnceLock;
    use std::time::Instant;
    use zxc::*;

    /// Compares the cost of the old `OnceLock<&'static>` config access with
    /// the current `RwLock<Option<Arc<..>>>` one. Run with:
    ///   cargo test --test config --release -- --ignored --nocapture
    #[test]
    #[ignore = "micro-benchmark, run manually"]
    fn config_access_overhead() {
        const CALLS: usize = 10_000_000;

        static BASELINE: OnceLock<RootConfig> = OnceLock::new();
        BASELINE.set(config().as_ref().clone()).unwrap();

        let start = Instant::now();
        let mut acc = 0.0f32;
        for _ in 0..CALLS {
            acc += std::hint::black_box(BASELINE.get().unwrap().pawn.speed);
        }
        let oncelock = start.elapsed();

        let start = Instant::now();
        for _ in 0..CALLS {
            acc += std::hint::black_box(config().pawn.speed);
        }
        let rwlock_arc = start.elapsed();

        std::hint::black_box(acc);
        println!(
            "{CALLS} calls:\n  OnceLock &'static: {oncelock:?} ({:.2}ns/call)\n  RwLock<Arc>:       {rwlock_arc:?} ({:.2}ns/call)",
            oncelock.as_secs_f64() * 1e9 / CALLS as f64,
            rwlock_arc.as_secs_f64() * 1e9 / CALLS as f64,
        );
    }
}
