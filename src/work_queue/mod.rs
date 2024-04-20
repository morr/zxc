use crate::*;

expose_submodules!(components, systems);

pub struct WorkQueuePlugin;

impl Plugin for WorkQueuePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorkQueue>()
           .add_systems(FixedUpdate, assign_tasks_to_pawns.run_if(in_state(WorldState::Playing)));
    }
}
