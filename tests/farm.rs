mod apply_global_config;

mod farm {
    use zxc::*;

    #[test]
    fn not_tended_yield() {
        let farm = Farm {
            state: FarmState::Grown,
            tendings_done: 0,
        };

        assert_eq!(
            farm.yield_amount(),
            (config().farming.max_yield * config().farming.basic_yield_percent).round() as u32
        );
    }

    #[test]
    fn half_tended_yield() {
        let farm = Farm {
            state: FarmState::Grown,
            tendings_done: (config().farming.growth_days / 2.0) as u32,
        };

        assert_eq!(
            farm.yield_amount(),
            (config().farming.max_yield
                * (config().farming.basic_yield_percent
                    + (1.0 - config().farming.basic_yield_percent) * 0.5))
                .round() as u32
        );
    }

    #[test]
    fn exactly_tended_yield() {
        let farm = Farm {
            state: FarmState::Grown,
            tendings_done: config().farming.growth_days as u32,
        };

        assert_eq!(farm.yield_amount(), config().farming.max_yield as u32);
    }

    #[test]
    fn overly_tended_yield() {
        let farm = Farm {
            state: FarmState::Grown,
            tendings_done: 99,
        };

        assert_eq!(farm.yield_amount(), config().farming.max_yield as u32);
    }
}
