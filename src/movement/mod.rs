use crate::*;

expose_submodules!(components, systems);

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Movement>()
            .add_event::<EntityStateChangeEvent<MovementState>>()
            .add_systems(Update, move_moving_entities.run_if(in_state(TimeState::Running)));
    }
}
