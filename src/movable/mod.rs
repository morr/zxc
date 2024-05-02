use crate::*;

expose_submodules!(components, systems);

pub struct MovablePlugin;

impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Movable>()
            .add_event::<EntityStateChangeEvent<MovableState>>()
            .add_systems(
                Update,
                move_moving_entities.run_if(in_state(SimulationState::Running)),
            );
    }
}
