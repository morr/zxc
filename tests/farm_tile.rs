use zxc::{
    structure::{FarmTile, FarmTileState},
    CONFIG,
};

#[test]
fn farm_tile_not_tended_yield() {
    let farm_tile = FarmTile {
        state: FarmTileState::Grown,
        tendings_done: 0,
    };

    assert_eq!(
        farm_tile.yield_amount(),
        (CONFIG.farming.basic_yield_percent * CONFIG.farming.max_yield).round() as u32
    );
}

#[test]
fn farm_tile_fully_tended_yield() {
    let farm_tile = FarmTile {
        state: FarmTileState::Grown,
        tendings_done: 99,
    };

    assert_eq!(
        farm_tile.yield_amount(),
        CONFIG.farming.max_yield as u32
    );
}
