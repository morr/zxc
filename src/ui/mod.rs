use crate::*;

expose_submodules!(simulation_state, items_stock, selectable, debug);

pub static UI_COLOR: Lazy<Color> = Lazy::new(|| Color::hex("181a1c").unwrap());

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiSimulationStatePlugin,
            UiItemsStockPlugin,
            UiSelectablePlugin,
            UiDebugPlugin,
        ));
    }
}

pub enum UiOpacity {
    Light,
    Medium,
    Heavy
}

pub fn ui_color(opacity: UiOpacity) -> Color {
    *UI_COLOR.clone().set_a(
        match opacity {
            UiOpacity::Light => 0.25,
            UiOpacity::Medium => 0.65,
            UiOpacity::Heavy => 0.85
        }
    )
}

pub fn bg_color(opacity: UiOpacity) -> BackgroundColor {
    ui_color(opacity).into()
}
