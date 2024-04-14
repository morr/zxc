use crate::*;

expose_submodules!(components, systems);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Movement>()
            .add_systems(Update, apply_movement.run_if(in_state(TimeState::Running)));
    }
}
