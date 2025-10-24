use super::*;

expose_submodules!(components, systems, utils);

pub struct DebugNoisePlugin;
impl Plugin for DebugNoisePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(DebugNoiseState::Hidden)
            .add_observer(on_rebuild_map_complete)
            .add_observer(on_debug_noise_state_change)
            .add_systems(
                OnExit(AppState::Loading),
                insert_unsynced_noise_texture.after(generate_map),
            );

        let init_noise_state = config().debug.noise_state.clone();
        if init_noise_state != DebugNoiseState::Hidden {
            app.add_systems(
                OnExit(AppState::Loading),
                (move |mut next_state: ResMut<NextState<DebugNoiseState>>, mut commands: Commands| {
                    next_state.set(init_noise_state.clone());
                    commands.trigger(log_event!(StateChangeEvent(init_noise_state.clone())));
                })
                .after(generate_map), // .after(initialize_noise_texture),
            );
        }
    }
}
