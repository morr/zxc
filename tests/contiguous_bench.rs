mod apply_default_global_config;

// Micro-benchmark for TODO 3.1: regular Query iteration vs contiguous_iter_mut
// over the same tick-style arithmetic. Run with:
//   cargo test --test contiguous_bench -- --ignored --nocapture
mod contiguous_bench {
    use std::time::Instant;

    use zxc::feedable::Feedable;
    use zxc::prelude::*;
    use zxc::restable::Restable;

    const ENTITIES: usize = 2000;
    const ITERATIONS: usize = 10_000;

    #[test]
    #[ignore = "micro-benchmark, run manually with --ignored --nocapture"]
    fn iter_vs_contiguous_iter() {
        let mut world = World::new();
        for _ in 0..ENTITIES {
            world.spawn((Feedable::default(), Restable::default()));
        }
        let mut query = world.query::<(&mut Feedable, &mut Restable)>();

        // warmup
        for (mut feedable, _) in query.iter_mut(&mut world) {
            feedable.hunger = 100.0;
        }

        let start = Instant::now();
        for _ in 0..ITERATIONS {
            for (mut feedable, mut restable) in query.iter_mut(&mut world) {
                feedable.hunger = (feedable.hunger - 0.001).max(0.0);
                restable.fatigue = (restable.fatigue + 0.001).min(100.0);
            }
        }
        let regular = start.elapsed();

        let start = Instant::now();
        for _ in 0..ITERATIONS {
            for (mut feedables, mut restables) in query
                .contiguous_iter_mut(&mut world)
                .expect("query is dense")
            {
                for (feedable, restable) in feedables.iter_mut().zip(restables.iter_mut()) {
                    feedable.hunger = (feedable.hunger - 0.001).max(0.0);
                    restable.fatigue = (restable.fatigue + 0.001).min(100.0);
                }
            }
        }
        let contiguous = start.elapsed();

        let per_pass_regular = regular.as_secs_f64() * 1e6 / ITERATIONS as f64;
        let per_pass_contiguous = contiguous.as_secs_f64() * 1e6 / ITERATIONS as f64;
        println!(
            "{ENTITIES} entities, {ITERATIONS} passes:\n  regular:    {regular:?} ({per_pass_regular:.2}us/pass)\n  contiguous: {contiguous:?} ({per_pass_contiguous:.2}us/pass)\n  speedup:    x{:.2}",
            per_pass_regular / per_pass_contiguous
        );
    }
}
