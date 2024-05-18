use crate::*;

expose_submodules!(components, systems);

pub struct CommandablePlugin;

impl Plugin for CommandablePlugin {
    fn build(&self, app: &mut App) {
        app
            // .register_type::<Commandable>()
            .add_systems(
                Update,
                process_commands
                    .run_if(in_state(AppState::Playing))
                    .run_if(in_state(SimulationState::Running)),
            );
    }
}
