use crate::*;

expose_submodules!(components, systems, pathfinding_algo, utils);

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArcNavmesh>()
            .add_message::<PathfindRequestMessage>()
            .add_message::<PathfindAnswerMessage>()
            .add_message::<OccupationChangeMessage>()
            // .add_systems(
            //     FixedUpdate,
            //     pathfinding_algo::measure_pathfinding.run_if(in_state(WorldState::Playing)),
            // )
            .add_systems(
                Update,
                (
                    listen_for_pathfinding_requests,
                    move_user_selected_pawn_on_click_stage_1,
                    listen_for_pathfinding_answers,
                    listen_for_pathfinding_async_tasks,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
