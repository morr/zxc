use crate::*;

expose_submodules!(components);
// expose_submodules!(components, systems);

pub struct RestablePlugin;

impl Plugin for RestablePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Restable>();
            // .add_event::<WorkCompleteEvent>()
            // .add_event::<WorkStartEvent>()
            // .add_systems(
            //     FixedUpdate,
            //     progress_work.run_if(in_state(SimulationState::Running)),
            // )
            // .add_systems(
            //     FixedUpdate,
            //     (
            //         assign_tasks_to_pawns,
            //         check_pawn_ready_for_working,
            //         start_pawn_working,
            //         complete_pawn_working,
            //     )
            //         .chain()
            //         .run_if(in_state(AppState::Playing)),
            // );
    }
}

