use crate::*;

mod components;
mod systems;
pub use self::{components::*, systems::*};

pub struct UserSelectPlugin;

impl Plugin for UserSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentUserSelection>()
            .add_observer(on_click_stage0.run_if(in_state(AppState::Playing)));
    }
}
