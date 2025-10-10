use crate::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            ai_idle_pawns
                .run_if(in_state(AppState::Playing))
                .run_if(in_state(SimulationState::Running)),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn ai_idle_pawns(
    mut commands: Commands,
    mut commandable_query: Query<
        (
            Entity,
            &Pawn,
            &Movable,
            &Restable,
            &Feedable,
            &mut Commandable,
            &Transform,
        ),
        (
            With<pawn_state::PawnStateIdleTag>,
            // With<commandable_state::CommandableStateIdleTag>,
        ),
    >,
    mut workable_query: Query<&Transform>,
    mut carryable_query: Query<&Transform>,
    mut tasks_queue: ResMut<TasksQueue>,
    arc_navmesh: Res<ArcNavmesh>,
    food_stock: Res<FoodStock>,
) {
    for (commandable_entity, pawn, movable, restable, feedable, mut commandable, transform) in
        &mut commandable_query
    {
        ensure_state!(loop: PawnState::Idle, pawn.state);
        continue_unless!(CommandableState::Idle, commandable.state);

        if feedable.is_overflowed() && food_stock.amount > 0 {
            commandable.set_queue(
                CommandType::Feed(FeedCommand { commandable_entity }),
                commandable_entity,
                &mut commands,
            );
        } else if restable.is_overflowed() {
            commandable.set_queue(
                CommandType::ToRest(ToRestCommand { commandable_entity }),
                commandable_entity,
                &mut commands,
            );
        } else if let Some(task) = tasks_queue.get_task() {
            let maybe_commands_sequence = match task.0 {
                TaskKind::Work {
                    workable_entity,
                    ref work_kind,
                } => {
                    let transform = workable_query
                        .get_mut(workable_entity)
                        .unwrap_or_else(|err| {
                            panic!(
                                "Failed to get query result for workable_entity {:?} {:?}",
                                workable_entity, err
                            )
                        });

                    Some(vec![
                        CommandType::MoveTo(MoveToCommand {
                            commandable_entity,
                            grid_tile: transform.world_pos_to_grid(),
                        }),
                        CommandType::WorkOn(WorkOnCommand {
                            commandable_entity,
                            workable_entity,
                            work_kind: work_kind.clone(),
                        }),
                        CommandType::CompleteTask(CompleteTaskCommand {
                            commandable_entity,
                            task,
                        }),
                    ])
                }
                TaskKind::CarryItem {
                    carryable_entity,
                    destination_grid_tile: grid_tile,
                } => {
                    if let Ok(transform) = carryable_query.get_mut(carryable_entity) {
                        Some(vec![
                            CommandType::MoveTo(MoveToCommand {
                                commandable_entity,
                                grid_tile: transform.world_pos_to_grid(),
                            }),
                            CommandType::PickUpItem(PickUpItemCommand {
                                commandable_entity,
                                carryable_entity,
                            }),
                            CommandType::MoveTo(MoveToCommand {
                                commandable_entity,
                                grid_tile,
                            }),
                            CommandType::DropCarriedItem(DropCarriedItemCommand {
                                commandable_entity,
                                carryable_entity,
                            }),
                            CommandType::CompleteTask(CompleteTaskCommand {
                                commandable_entity,
                                task,
                            }),
                        ])
                    } else {
                        None
                    }
                }
            };

            if let Some(commands_sequence) = maybe_commands_sequence {
                commandable.set_queue(commands_sequence, commandable_entity, &mut commands);
            }
        } else {
            if !config().pawn.wander_when_idle {
                continue;
            }
            if movable.state != MovableState::Idle {
                continue;
            }
            let mut rng = rand::rng();

            let world_pos = transform.translation.truncate();
            let end_tile = find_empty_grid_tile(world_pos, &arc_navmesh.read(), &mut rng, 0);

            commandable.set_queue(
                CommandType::MoveTo(MoveToCommand {
                    commandable_entity,
                    grid_tile: end_tile,
                }),
                commandable_entity,
                &mut commands,
            );
        }
    }
}
