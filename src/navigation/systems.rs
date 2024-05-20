use bevy::tasks::{block_on, futures_lite::future};

use super::*;

pub fn move_user_selected_pawn_on_click_stage_1(
    mut commands: Commands,
    mut click_event_reader: EventReader<ClickEventStage1>,
    user_selection: Res<CurrentUserSelection>,
    mut pawn_query: Query<
        &mut Commandable,
        Or<(With<pawn_state::Idle>, With<pawn_state::ExecutingCommand>)>,
    >,
    // mut pawn_query: Query<
    //     (&Transform, &mut Movable, &mut Commandable, Option<&mut PathfindingTask>),
    //     (With<pawn_state::Idle>, With<UserSelectionMarker>),
    // >,
    // mut movable_state_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
) {
    for ClickEventStage1(grid_tile) in click_event_reader.read() {
        println!("===== CLICK {:?} =====", *grid_tile);
        let Some(UserSelectionData { entity, kind }) = &user_selection.0 else {
            continue;
        };
        let UserSelectionKind::Pawn = kind else {
            continue;
        };
        let Ok(mut commandable) = pawn_query.get_mut(*entity) else {
            continue;
        };

        commandable.schedule_execution(
            CommandType::MoveTo(MoveToCommand(*entity, *grid_tile)),
            *entity,
            &mut commands,
        );

        // movable.to_pathfinding_async(
        //     *id,
        //     transform.translation.truncate().world_pos_to_grid(),
        //     *grid_tile,
        //     &arc_navmesh,
        //     &queue_counter,
        //     maybe_pathfinding_task.as_deref_mut(),
        //     &mut commands,
        //     &mut movable_state_event_writer,
        // );
    }
}

// pub fn pathfinding_on_click(
//     mut commands: Commands,
//     mut click_event_reader: EventReader<ClickTileEvent>,
//     mut query_pawns: Query<(Entity, &Transform, &mut Movable), With<Movable>>,
//     mut pathfind_event_writer: EventWriter<PathfindRequestEvent>,
//     mut movable_state_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
// ) {
//     for click_event in click_event_reader.read() {
//         for (entity, transform, mut movable) in &mut query_pawns {
//             movable.to_pathfinding(
//                 entity,
//                 transform.translation.truncate().world_pos_to_grid(),
//                 click_event.0,
//                 &mut commands,
//                 &mut pathfind_event_writer,
//                 &mut movable_state_event_writer,
//             );
//         }
//     }
// }

pub fn listen_for_pathfinding_requests(
    arc_navmesh: Res<ArcNavmesh>,
    mut pathfind_event_reader: EventReader<PathfindRequestEvent>,
    mut pathfind_event_writer: EventWriter<PathfindAnswerEvent>,
) {
    for event in pathfind_event_reader.read() {
        // println!("{:?}", event);

        let path = pathfinding_algo::astar_pathfinding(
            &arc_navmesh.read(),
            &event.start_tile,
            &event.end_tile,
        );

        pathfind_event_writer.send(PathfindAnswerEvent {
            entity: event.entity,
            start_tile: event.start_tile,
            end_tile: event.end_tile,
            path,
        });
    }
}

pub fn listen_for_pathfinding_async_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut Movable, &mut PathfindingTask), With<PathfindingTask>>,
    mut event_writer: EventWriter<MovableReachedDestinationEvent>,
    // mut event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
) {
    for (entity, mut movable, mut pathfinding_tasks) in &mut tasks {
        pathfinding_tasks.0.retain_mut(|task| {
            if let Some(result) = block_on(future::poll_once(task)) {
                // println!("{:?}", task);
                if let MovableState::Pathfinding(end_tile) = movable.state {
                    // check if it an is outdated pathfinding answer
                    if end_tile != result.end_tile {
                        return false;
                    }

                    if let Some(path) = &result.path {
                        // pathfindign always return tile of current location,
                        // so if it has returned only one IVec2,
                        // then it means that we are pathfinding path to our
                        // current location. no movement needed
                        if path.len() == 1 {
                            println!("EventWriter<MovableReachedDestinationEvent> from listen_for_pathfinding_async_tasks");
                            movable.to_idle(entity, &mut commands, Some(&mut event_writer));
                        } else {
                            movable.to_moving(
                                end_tile,
                                path.iter().skip(1).cloned().collect(),
                                entity,
                                &mut commands,
                                // &mut event_writer,
                            );
                        }
                    } else {
                        movable.to_pathfinding_error(entity, end_tile, &mut commands/*, &mut event_writer*/);
                    }
                } else {
                    // println!(
                    //     "movable.state != MovableState::Pathfinding, movable.state={:?}",
                    //     movable.state
                    // );
                }
                false // remove the task
            } else {
                true
            }
        });

        if pathfinding_tasks.0.is_empty() {
            commands.entity(entity).remove::<PathfindingTask>();
        }
    }
}

pub fn listen_for_pathfinding_answers(
    mut commands: Commands,
    mut pathfind_event_reader: EventReader<PathfindAnswerEvent>,
    mut query_movable: Query<(Entity, &mut Movable), With<Movable>>,
    mut event_writer: EventWriter<MovableReachedDestinationEvent>,
    // mut event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
) {
    for event in pathfind_event_reader.read() {
        // println!("{:?}", event);

        let Ok((entity, mut movable)) = query_movable.get_mut(event.entity) else {
            continue;
        };
        if let MovableState::Pathfinding(end_tile) = movable.state {
            // check if it an is outdated pathfinding answer
            if end_tile != event.end_tile {
                // println!(
                //     "end_tile != event.end, end_tile={}, event.end={}",
                //     end_tile, event.end
                // );
                return;
            }

            if let Some(path) = &event.path {
                // pathfindign always return tile of current location,
                // so if it has returned only one IVec2,
                // then it means that we are pathfinding path to our
                // current location. no movement needed
                if path.len() == 1 {
                    println!("EventWriter<MovableReachedDestinationEvent> from listen_for_pathfinding_answers");
                    movable.to_idle(entity, &mut commands, Some(&mut event_writer));
                } else {
                    movable.to_moving(
                        event.end_tile,
                        path.iter().skip(1).cloned().collect(),
                        entity,
                        &mut commands,
                        // &mut event_writer,
                    );
                }
            } else {
                movable.to_pathfinding_error(entity, event.end_tile, &mut commands/*, &mut event_writer*/);
            }
        } else {
            // println!(
            //     "movable.state != MovableState::Pathfinding, movable.state={:?}",
            //     movable.state
            // );
        }
    }
}
