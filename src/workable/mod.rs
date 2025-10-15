use crate::*;

expose_submodules!(components, systems);

pub struct WorkablePlugin;

impl Plugin for WorkablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Workable>()
            .add_observer(on_work_complete)
            // .add_message::<WorkStartEvent>()
            .add_systems(
                FixedUpdate,
                progress_work.run_if(in_state(AppState::Playing)),
            );
    }
}
