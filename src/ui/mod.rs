use crate::*;

expose_submodules!(simulation_state, items_stock, pawn, farm, debug);

pub static UI_COLOR: Lazy<Color> = Lazy::new(|| Color::hex("181a1c").unwrap());

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiSimulationStatePlugin,
            UiItemsStockPlugin,
            UiPawnPlugin,
            UiFarmPlugin,
            UiDebugPlugin,
        ));
    }
}
