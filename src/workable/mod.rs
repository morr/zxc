use crate::*;

expose_submodules!(components);

// expose_submodules!(components, systems);

pub struct WorkablePlugin;

impl Plugin for WorkablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Workable>();
        //     .add_event::<EntityStateChangeEvent<WorkableState>>()
        //     .add_systems(
        //         Update,
        //         move_moving_entities.run_if(in_state(TimeState::Running)),
        //     );
    }
}
