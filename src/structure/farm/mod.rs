use super::*;

expose_submodules!(components, systems, debug_components);

pub struct FarmPlugin;

impl Plugin for FarmPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Farm>()
            .add_message::<FarmProgressMessage>()
            .add_message::<FarmTendedMessage>()
            .add_observer(self::systems::on_new_day)
            .add_observer(on_farm_state_change)
            .add_systems(
                FixedUpdate,
                (
                    progress_on_farm_progress_event,
                    progress_on_farm_tended_event,
                    progress_planted_and_tending_rest_timers,
                    progress_harvested_timer,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
