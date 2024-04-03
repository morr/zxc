use crate::*;

expose_submodules!(components, systems, pathfinding_algo);

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArcNavmesh>()
            .add_event::<PathfindRequestEvent>()
            .add_event::<PathfindAnswerEvent>()
            // .add_systems(
            //     FixedUpdate,
            //     pathfinding_algo::measure_pathfinding.run_if(in_state(WorldState::Playing)),
            // )
            .add_systems(
                Update,
                (
                    listen_for_pathfinding_requests,
                    pathfinding_on_click,
                    listen_for_pathfinding_answers,
                )
                    .chain()
                    .run_if(in_state(WorldState::Playing)),
            );
    }
}
