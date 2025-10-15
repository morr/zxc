use crate::*;

expose_submodules!(components, systems, utils);

pub struct StoryTimePlugin;

impl Plugin for StoryTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<NewDayMessage>()
            .add_systems(FixedUpdate, track_time.run_if(in_state(AppState::Playing)))
            .add_systems(Update, modify_time.run_if(in_state(AppState::Playing)));
    }
}
