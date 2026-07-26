use crate::*;

mod components;
mod systems;
pub use self::{components::*, systems::*};

pub struct WorkablePlugin;

impl Plugin for WorkablePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_work_complete.run_if(in_state(AppState::Playing)))
            .add_systems(
                FixedUpdate,
                progress_work.run_if(in_state(AppState::Playing)),
            );
    }
}
