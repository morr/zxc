use bevy::tasks::{block_on, poll_once, AsyncComputeTaskPool, Task};

use super::*;

#[derive(Component)]
pub struct PathfindingTask {
    task: Task<Option<Vec<IVec2>>>,
    pub start: IVec2,
    pub end: IVec2,
}

pub fn pathfinding_on_click(
    mut commands: Commands,
    mut click_event_reader: EventReader<ClickTileEvent>,
    mut query_pawns: Query<(Entity, &Transform), With<Movement>>,
    arc_navmesh: Res<ArcNavmesh>,
) {
    for click_event in click_event_reader.read() {
        for (entity, transform) in &mut query_pawns {
            let start = transform.translation.truncate().world_pos_to_grid();
            let end = click_event.0;

            let navmesh_arc = arc_navmesh.0.clone();
            let thread_pool = AsyncComputeTaskPool::get();

            let task = thread_pool.spawn(async move {
                let navmesh = navmesh_arc.read().unwrap();
                astar_pathfinding(&navmesh, &start, &end)
            });

            commands
                .entity(entity)
                .insert(PathfindingTask { task, start, end });
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

        let path =
            pathfinding_algo::astar_pathfinding(&arc_navmesh.read(), &event.start, &event.end);

        pathfind_event_writer.send(PathfindAnswerEvent {
            entity: event.entity,
            start: event.start,
            end: event.end,
            path,
        });
    }
}

pub fn listen_for_pathfinding_tasks(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut Movement, &mut PathfindingTask)>,
    mut movement_state_event_writer: EventWriter<EntityStateChangeEvent<MovementState>>,
) {
    for (entity, mut movement, mut task) in &mut tasks {
        if let Some(result) = block_on(poll_once(&mut task.task)) {
            commands.entity(entity).remove::<PathfindingTask>();

            if let MovementState::Pathfinding(end_tile) = movement.state {
                // check if it an is outdated pathfinding answer
                if end_tile != task.end {
                    // println!(
                    //     "end_tile != task.end, end_tile={}, task.end={}",
                    //     end_tile, task.end
                    // );
                    return;
                }

                if let Some(path) = &result {
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
            if end_tile != event.end {
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
