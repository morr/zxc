use crate::*;

expose_submodules!(components, systems);

pub struct WorkablePlugin;

impl Plugin for WorkablePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorkQueue>()
            .register_type::<Workable>()
            .add_event::<WorkCompleteEvent>()
            .add_event::<WorkStartEvent>()
            .add_systems(
                FixedUpdate,
                progress_work.run_if(in_state(TimeState::Running)),
            )
            .add_systems(
                FixedUpdate,
                (
                    assign_tasks_to_pawns,
                    check_pawn_ready_for_working,
                    start_pawn_working,
                    complete_pawn_working,
                )
                    .chain()
                    .run_if(in_state(WorldState::Playing)),
            );
    }
}
