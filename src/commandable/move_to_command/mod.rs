use super::*;

pub struct MoveToCommandPlugin;

impl Plugin for MoveToCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveToCommand>().add_systems(
            Update,
            (execute_command, monitor_completion)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Event, Debug, Clone)]
pub struct MoveToCommand(pub Entity, pub IVec2);

fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<MoveToCommand>,
    mut movable_query: Query<(
        &Transform,
        &mut Movable,
        &mut Commandable,
        Option<&mut PathfindingTask>,
    )>,
    arc_navmesh: Res<ArcNavmesh>,
    queue_counter: Res<AsyncQueueCounter>,
    // mut movable_state_change_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
) {
    for MoveToCommand(ref entity, ref grid_tile) in command_reader.read() {
        // println!("{:?}", command);
        let (transform, mut movable, mut commandable, mut maybe_pathfinding_task) =
            movable_query.get_mut(*entity).unwrap();

        movable.to_pathfinding_async(
            *entity,
            transform.translation.truncate().world_pos_to_grid(),
            *grid_tile,
            &arc_navmesh,
            &queue_counter,
            maybe_pathfinding_task.as_deref_mut(),
            &mut commands,
            // &mut movable_state_change_event_writer,
        );
        commandable.change_state(CommandableState::Executing, *entity, &mut commands);
    }
}

fn monitor_completion(
    mut commands: Commands,
    mut query: Query<&mut Commandable, With<commandable_state::Executing>>,
    mut command_event_reader: EventReader<MovableReachedDestinationEvent>,
    mut commandable_event_writer: EventWriter<CommandExecutedEvent>,
) {
    for MovableReachedDestinationEvent(ref entity, ref destination_tile) in
        command_event_reader.read()
    {
        let Ok(mut commandable) = query.get_mut(*entity) else {
            continue;
        };
        let Some(ref command_type) = commandable.executing else {
            continue;
        };
        let CommandType::MoveTo(MoveToCommand(ref _entity, ref move_to_tile)) = command_type else {
            continue;
        };
        if destination_tile != move_to_tile {
            continue;
        }

        commandable.complete_execution(*entity, &mut commands, &mut commandable_event_writer);
    }
}
