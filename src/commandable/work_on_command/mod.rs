use super::*;

pub struct WorkOnCommandPlugin;

impl Plugin for WorkOnCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WorkOnCommand>().add_systems(
            Update,
            (execute_command)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Event, Debug, Clone)]
pub struct WorkOnCommand(pub Entity, pub IVec2);

fn execute_command(
    // mut commands: Commands,
    // mut command_reader: EventReader<MoveToCommand>,
    // mut movable_query: Query<(
    //     &Transform,
    //     &mut Movable,
    //     &mut Commandable,
    //     Option<&mut PathfindingTask>,
    // )>,
    // arc_navmesh: Res<ArcNavmesh>,
    // queue_counter: Res<AsyncQueueCounter>,
    // mut movable_state_change_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
) {
}
