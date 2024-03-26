use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugNavmeshState {
    // MainMenu,
    #[default]
    Hidden,
    Visible,
}

#[derive(Debug, Event)]
pub struct StateChangeEvent<T>(pub T);

pub struct DebugNavMeshPlugin;
impl Plugin for DebugNavMeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StateChangeEvent<DebugNavmeshState>>()
            .init_state::<DebugNavmeshState>()
            .add_systems(FixedUpdate, handle_state_changes);

        // .add_systems(handle_state_changes)

        //     Update,
        //     render_grid.run_if(in_state(DebugGridState::Visible)),
        // )
        // .init_state::<DebugGridState>();
    }
}

fn handle_state_changes(mut event_reader: EventReader<StateChangeEvent<DebugNavmeshState>>) {
    for event in event_reader.read() {
        println!("{:?}", event);
    }
}
