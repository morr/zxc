use std::collections::VecDeque;

use crate::*;

pub struct CommandablePlugin;

impl Plugin for CommandablePlugin {
    fn build(&self, _app: &mut App) {
        // app.register_type::<Commandable>()
        //     .add_systems(Update, progress_stamina.run_if(in_state(AppState::Playing)));
    }
}

// pub struct Command<T: Event>(T);

// #[derive(Component, Debug, Default, InspectorOptions, Reflect)]
// #[reflect(InspectorOptions)]
#[derive(Component, Debug, Default)]
pub struct Commandable {
    // pub queue: VecDeque<Event>,
}
