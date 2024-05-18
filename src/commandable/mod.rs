
use crate::*;

pub struct CommandablePlugin;

impl Plugin for CommandablePlugin {
    fn build(&self, _app: &mut App) {
        // app.register_type::<Commandable>()
        //     .add_systems(Update, progress_stamina.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Commandable {
}
