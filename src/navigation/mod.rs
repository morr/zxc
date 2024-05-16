use crate::*;

expose_submodules!(components, systems, pathfinding_algo, utils);

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArcNavmesh>()
            .add_event::<PathfindRequestEvent>()
            .add_event::<PathfindAnswerEvent>()
            .add_event::<OccupationChangeEvent>()
            // .add_systems(
            //     FixedUpdate,
            //     pathfinding_algo::measure_pathfinding.run_if(in_state(WorldState::Playing)),
            // )
            .add_systems(
                Update,
                (
                    listen_for_pathfinding_requests,
                    pathfinding_async_on_click,
                    listen_for_pathfinding_answers,
                    listen_for_pathfinding_async_tasks,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
