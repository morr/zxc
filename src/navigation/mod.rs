use crate::*;

expose_submodules!(components, systems, pathfinding_algo, utils);

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArcNavmesh>()
            .add_observer(on_pathfinding_request)
            .add_observer(on_pathfinding_answer)
            .add_message::<OccupationChangeMessage>()
            // .add_systems(
            //     FixedUpdate,
            //     pathfinding_algo::measure_pathfinding.run_if(in_state(WorldState::Playing)),
            // )
            .add_systems(
                Update,
                (
                    move_user_selected_pawn_on_click_stage_1,
                    listen_for_pathfinding_async_tasks,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
