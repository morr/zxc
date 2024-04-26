use crate::*;

expose_submodules!(components, systems);

pub struct WorkablePlugin;

impl Plugin for WorkablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Workable>().add_systems(
            FixedUpdate,
            progress_work.run_if(in_state(TimeState::Running)),
        );
    }
}
