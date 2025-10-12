use super::*;

pub struct MoveToCommandPlugin;

impl Plugin for MoveToCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MoveToCommand>()
            .add_observer(on_internal_interrupt)
            .add_observer(on_movable_reached_destination)
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Message, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct MoveToCommand {
    pub commandable_entity: Entity,
    pub grid_tile: IVec2,
}

fn execute_command(
    mut commands: Commands,
    mut command_reader: MessageReader<MoveToCommand>,
    mut query: Query<(&Transform, &mut Movable, Option<&mut PathfindingTask>)>,
    arc_navmesh: Res<ArcNavmesh>,
    queue_counter: Res<AsyncQueueCounter>,
    // mut movable_state_change_event_writer: MessageWriter<EntityStateChangeMessage<MovableState>>,
) {
    for MoveToCommand {
        commandable_entity,
        grid_tile,
    } in command_reader.read()
    {
        // println!("{:?}", MoveToCommand(entity, grid_tile));
        match query.get_mut(*commandable_entity) {
            Ok((transform, mut movable, mut maybe_pathfinding_task)) => {
                movable.to_pathfinding_async(
                    *commandable_entity,
                    transform.translation.truncate().world_pos_to_grid(),
                    *grid_tile,
                    &arc_navmesh,
                    &queue_counter,
                    maybe_pathfinding_task.as_deref_mut(),
                    &mut commands,
                    // &mut movable_state_change_event_writer,
                );
                // commandable.change_state(CommandableState::Executing, *entity, &mut commands);
            }
            Err(err) => {
                warn!("Failed to get query result: {:?}", err);
                continue;
            }
        }
    }
}

fn on_movable_reached_destination(
    event: On<MovableReachedDestinationEvent>,
    mut commands: Commands,
    mut query: Query<&mut Commandable>,
) {
    let MovableReachedDestinationEvent { entity, grid_tile: destination_tile } = *event;

    // println!("{:?}", MovableReachedDestinationEvent(*entity, *destination_tile));
    let Ok(mut commandable) = query.get_mut(entity) else {
        return;
    };
    let Some(ref command_type) = commandable.executing else {
        return;
    };
    let CommandType::MoveTo(MoveToCommand {
        commandable_entity: _,
        grid_tile: move_to_tile,
    }) = command_type
    else {
        return;
    };
    if destination_tile != *move_to_tile {
        return;
    }

    commandable.complete_executing(entity, &mut commands);
}

fn on_internal_interrupt(
    event: On<InternalCommandInterruptEvent>,
    mut commands: Commands,
    mut query: Query<&mut Movable>,
) {
    let CommandType::MoveTo(MoveToCommand {
        commandable_entity,
        grid_tile: commanding_to_tile,
    }) = event.command_type
    else {
        return;
    };
    let Ok(mut movable) = query.get_mut(commandable_entity) else {
        return;
    };

    if let MovableState::Moving(moving_to_tile)
    | MovableState::Pathfinding(moving_to_tile)
    | MovableState::PathfindingError(moving_to_tile) = movable.state
    {
        if moving_to_tile == commanding_to_tile {
            movable.to_idle(commandable_entity, &mut commands, false);
        } else {
            warn!(
                "Attempt to interrupt MoveTo({:?}) for Movable({:?})",
                commanding_to_tile, moving_to_tile
            );
        }
    }
}
