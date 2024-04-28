use crate::*;

expose_submodules!(components, systems);

pub struct StoryTimePlugin;

impl Plugin for StoryTimePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TimeState>()
            .init_resource::<TimeScale>()
            .init_resource::<ElapsedTime>()
            .add_systems(FixedUpdate, track_time.run_if(in_state(TimeState::Running)))
            .add_systems(Update, modify_time.run_if(in_state(WorldState::Playing)));
    }
}
