use crate::*;

expose_submodules!(components, systems);

pub struct MovablePlugin;

impl Plugin for MovablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Movable>()
            .add_observer(self::systems::on_pawn_death)
            // .add_message::<MovableReachedDestinationEvent>()
            // .add_message::<EntityStateChangeMessage<MovableState>>()
            .add_systems(
                Update,
                move_moving_entities.run_if(in_state(AppState::Playing)),
            );
    }
}
