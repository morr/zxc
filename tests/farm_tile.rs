use zxc::{
    structure::{Farm, FarmState},
    CONFIG,
};

#[test]
fn farm_not_tended_yield() {
    let farm = Farm {
        state: FarmState::Grown,
        tendings_done: 0,
    };

    assert_eq!(
        farm.yield_amount(),
        (CONFIG.farming.max_yield * CONFIG.farming.basic_yield_percent).round() as u32
    );
}

#[test]
fn farm_half_tended_yield() {
    let farm = Farm {
        state: FarmState::Grown,
        tendings_done: (CONFIG.farming.growth_days / 2.0) as u32,
    };

    assert_eq!(
        farm.yield_amount(),
        (CONFIG.farming.max_yield
            * (CONFIG.farming.basic_yield_percent
                + (1.0 - CONFIG.farming.basic_yield_percent) * 0.5))
            .round() as u32
    );
}

#[test]
fn farm_exactly_tended_yield() {
    let farm = Farm {
        state: FarmState::Grown,
        tendings_done: CONFIG.farming.growth_days as u32,
    };

    assert_eq!(farm.yield_amount(), CONFIG.farming.max_yield as u32);
}

#[test]
fn farm_overly_tended_yield() {
    let farm = Farm {
        state: FarmState::Grown,
        tendings_done: 99,
    };

    assert_eq!(farm.yield_amount(), CONFIG.farming.max_yield as u32);
}
