use crate::*;

expose_submodules!(components, systems, tasks_queue);

pub struct WorkablePlugin;

impl Plugin for WorkablePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TasksQueuePlugin)
            .register_type::<Workable>()
            .add_event::<WorkCompleteEvent>()
            // .add_event::<WorkStartEvent>()
            .add_systems(
                FixedUpdate,
                progress_work.run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                FixedUpdate,
                (
                    assign_tasks_to_pawns,
                    // start_pawn_working,
                    // complete_pawn_working,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
