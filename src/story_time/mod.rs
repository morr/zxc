use crate::*;

expose_submodules!(components, systems, utils);

pub struct StoryTimePlugin;

impl Plugin for StoryTimePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .init_resource::<TimeScale>()
            .init_resource::<ElapsedTime>()
            .add_message::<NewDayMessage>()
            .add_systems(
                FixedUpdate,
                track_time
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(Update, modify_time.run_if(in_state(AppState::Playing)));
    }
}
