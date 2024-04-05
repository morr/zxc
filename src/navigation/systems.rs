use bevy::tasks::{block_on, futures_lite::future};

use super::*;

pub fn pathfinding_async_on_click(
    arc_navmesh: Res<ArcNavmesh>,
    queue_counter: Res<AsyncQueueCounter>,
    mut commands: Commands,
    mut click_event_reader: EventReader<ClickTileEvent>,
    mut query_pawns: Query<(Entity, &Transform, &mut Movement), With<Movement>>,
    mut movement_state_event_writer: EventWriter<EntityStateChangeEvent<MovementState>>,
) {
    for click_event in click_event_reader.read() {
        for (entity, transform, mut movement) in &mut query_pawns {
            movement.to_pathfinding_async(
                entity,
                transform.translation.truncate().world_pos_to_grid(),
                click_event.0,
                &arc_navmesh,
                &queue_counter,
                &mut commands,
                &mut movement_state_event_writer,
            );
        }
    }
}

// pub fn pathfinding_on_click(
//     mut commands: Commands,
//     mut click_event_reader: EventReader<ClickTileEvent>,
//     mut query_pawns: Query<(Entity, &Transform, &mut Movement), With<Movement>>,
//     mut pathfind_event_writer: EventWriter<PathfindRequestEvent>,
//     mut movement_state_event_writer: EventWriter<EntityStateChangeEvent<MovementState>>,
// ) {
//     for click_event in click_event_reader.read() {
//         for (entity, transform, mut movement) in &mut query_pawns {
//             movement.to_pathfinding(
//                 entity,
//                 transform.translation.truncate().world_pos_to_grid(),
//                 click_event.0,
//                 &mut commands,
//                 &mut pathfind_event_writer,
//                 &mut movement_state_event_writer,
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
    mut tasks: Query<(Entity, &mut Movement, &mut PathfindingTask), With<PathfindingTask>>,
    mut movement_state_event_writer: EventWriter<EntityStateChangeEvent<MovementState>>,
) {
    for (entity, mut movement, mut pathfinding_tasks) in &mut tasks {
        pathfinding_tasks.0.retain(|task| {
            if let Some(result) = block_on(future::poll_once(&mut task)) {
            //     // println!("{:?}", task);
            //
            //     commands.entity(entity).remove::<PathfindingTask>();
            //
            //     if let MovementState::Pathfinding(end_tile) = movement.state {
            //         // check if it an is outdated pathfinding answer
            //         if end_tile != task.end_tile {
            //             // println!(
            //             //     "end_tile != task.end, end_tile={}, task.end={}",
            //             //     end_tile, task.end
            //             // );
            //             return;
            //         }
            //
            //         if let Some(path) = &result {
            //             if path.len() == 1 {
            //                 movement.to_idle(
            //                     entity,
            //                     &mut commands,
            //                     &mut movement_state_event_writer,
            //                 );
            //             } else {
            //                 movement.to_moving(
            //                     path.iter().skip(1).cloned().collect(),
            //                     entity,
            //                     &mut commands,
            //                     &mut movement_state_event_writer,
            //                 );
            //             }
            //         } else {
            //             movement.to_pathfinding_error(entity, &mut movement_state_event_writer);
            //         }
            //     } else {
            //         println!(
            //             "movement.state != MovementState::Pathfinding, movement.state={:?}",
            //             movement.state
            //         );
            //     }
                false
            } else {
                true
            }
        });
    }
}

pub fn listen_for_pathfinding_answers(
    mut commands: Commands,
    mut pathfind_event_reader: EventReader<PathfindAnswerEvent>,
    mut query_movement: Query<(Entity, &mut Movement), With<Movement>>,
    mut movement_state_event_writer: EventWriter<EntityStateChangeEvent<MovementState>>,
) {
    for event in pathfind_event_reader.read() {
        // println!("{:?}", event);

        let Ok((entity, mut movement)) = query_movement.get_mut(event.entity) else {
            continue;
        };
        if let MovementState::Pathfinding(end_tile) = movement.state {
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
                    movement.to_idle(entity, &mut commands, &mut movement_state_event_writer);
                } else {
                    movement.to_moving(
                        path.iter().skip(1).cloned().collect(),
                        entity,
                        &mut commands,
                        &mut movement_state_event_writer,
                    );
                }
            } else {
                movement.to_pathfinding_error(entity, &mut movement_state_event_writer);
            }
        } else {
            println!(
                "movement.state != MovementState::Pathfinding, movement.state={:?}",
                movement.state
            );
        }
    }
}
