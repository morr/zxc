use super::*;

expose_submodules!(pawn, farm);

pub struct UiSelectablePlugin;

impl Plugin for UiSelectablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiPawnPlugin,
            UiFarmPlugin,
        ));
    }
}
