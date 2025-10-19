use super::*;

expose_submodules!(components, systems);

pub struct DebugNoisePlugin;
impl Plugin for DebugNoisePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(DebugNoiseState::Hidden)
            .add_systems(
                OnExit(AppState::Loading),
                initialize_noise_texture.after(generate_map),
            )
            .add_observer(self::systems::on_rebuild_map)
            .add_observer(on_debug_noise_state_change);

        if config().debug.is_noise {
            app.add_systems(
                OnExit(AppState::Loading),
                (|mut next_state: ResMut<NextState<DebugNoiseState>>, mut commands: Commands| {
                    next_state.set(DebugNoiseState::Visible);
                    commands.trigger(log_event!(StateChangeEvent(DebugNoiseState::Visible)));
                })
                .after(generate_map)
                .after(initialize_noise_texture),
            );
        }
    }
}
