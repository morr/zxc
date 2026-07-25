use crate::*;

expose_submodules!(components, systems, pathfinding_algo, utils);

pub struct NavigationPlugin;
impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArcNavmesh>()
            .add_observer(on_pathfinding_request.run_if(in_state(AppState::Playing)))
            .add_observer(on_pathfinding_answer.run_if(in_state(AppState::Playing)))
            .add_observer(on_click_stage1.run_if(in_state(AppState::Playing)))
            .add_systems(
                Update,
                listen_for_pathfinding_async_tasks.run_if(in_state(AppState::Playing)),
            );
    }
}
