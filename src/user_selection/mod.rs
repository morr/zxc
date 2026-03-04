use crate::*;

expose_submodules!(components, systems);

pub struct UserSelectPlugin;

impl Plugin for UserSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentUserSelection>()
            .add_observer(on_click_stage0);
    }
}
