use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugNavmeshState {
    // MainMenu,
    #[default]
    Hidden,
    Visible,
}

pub struct DebugNavMeshPlugin;
impl Plugin for DebugNavMeshPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugNavmeshState>(); //.add_systems(
        //     Update,
        //     render_grid.run_if(in_state(DebugGridState::Visible)),
        // )
        // .init_state::<DebugGridState>();
    }
}
