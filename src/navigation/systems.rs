use bevy::tasks::{block_on, futures_lite::future};

use super::*;

// pub fn pathfinding_async_on_click(
//     arc_navmesh: Res<ArcNavmesh>,
//     queue_counter: Res<AsyncQueueCounter>,
//     mut commands: Commands,
//     mut click_event_reader: EventReader<ClickEvent>,
//     mut query_pawns: Query<
//         (
//             Entity,
//             &Transform,
//             &mut Movable,
//             Option<&mut PathfindingTask>,
//         ),
//         With<pawn_state::Idle>,
//         // With<Movable>,
//         // (With<Movable>, With<pawn_state::Idle>),
//     >,
//     mut movable_state_event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
// ) {
//     for click_event in click_event_reader.read() {
//         for (entity, transform, mut movable, mut maybe_pathfinding_task) in &mut query_pawns {
//             movable.to_pathfinding_async(
//                 entity,
//                 transform.translation.truncate().world_pos_to_grid(),
//                 click_event.0,
//                 &arc_navmesh,
//                 &queue_counter,
//                 maybe_pathfinding_task.as_deref_mut(),
//                 &mut commands,
//                 &mut movable_state_event_writer,
//             );
//         }
//     }
// }

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
    mut event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
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
                        if path.len() == 1 {
                            movable.to_idle(entity, &mut commands, Some(&mut event_writer));
                        } else {
                            movable.to_moving(
                                path.iter().skip(1).cloned().collect(),
                                entity,
                                &mut commands,
                                &mut event_writer,
                            );
                        }
                    } else {
                        movable.to_pathfinding_error(entity, &mut event_writer);
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
    mut event_writer: EventWriter<EntityStateChangeEvent<MovableState>>,
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
                if path.len() == 1 {
                    movable.to_idle(entity, &mut commands, Some(&mut event_writer));
                } else {
                    movable.to_moving(
                        path.iter().skip(1).cloned().collect(),
                        entity,
                        &mut commands,
                        &mut event_writer,
                    );
                }
            } else {
                movable.to_pathfinding_error(entity, &mut event_writer);
            }
        } else {
            // println!(
            //     "movable.state != MovableState::Pathfinding, movable.state={:?}",
            //     movable.state
            // );
        }
    }
}
