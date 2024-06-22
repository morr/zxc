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

fn ai_idle_pawns(
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &Pawn,
            &Movable,
            &Restable,
            &mut Commandable,
            &Transform,
        ),
        (
            With<pawn_state::PawnStateIdleTag>,
            // With<commandable_state::CommandableStateIdleTag>,
        ),
    >,
    mut tasks_queue: ResMut<TasksQueue>,
    mut commandable_interrupt_writer: EventWriter<InternalCommandInterruptEvent>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
    arc_navmesh: Res<ArcNavmesh>,
) {
    for (commandable_entity, pawn, movable, restable, mut commandable, transform) in &mut query {
        ensure_state!(PawnState::Idle, pawn.state);
        continue_unless!(CommandableState::Idle, commandable.state);

        if restable.is_overflowed() {
            commandable.set_queue(
                CommandType::ToRest(ToRestCommand { commandable_entity }),
                commandable_entity,
                &mut commands,
                &mut commandable_interrupt_writer,
                &mut tasks_scheduler,
            );
        } else if let Some(task) = tasks_queue.get_task() {
            let commands_sequence = match task.kind {
                TaskKind::Work { .. } => {
                    vec![
                        CommandType::MoveTo(MoveToCommand {
                            commandable_entity,
                            grid_tile: task.grid_tile,
                        }),
                        CommandType::WorkOn(WorkOnCommand {
                            commandable_entity,
                            task,
                        }),
                    ]
                }
                TaskKind::CarryItem {
                    carryable_entity,
                    grid_tile,
                } => {
                    vec![
                        CommandType::MoveTo(MoveToCommand {
                            commandable_entity,
                            grid_tile: task.grid_tile,
                        }),
                        CommandType::PickUp(PickUpCommand {
                            commandable_entity,
                            carryable_entity,
                        }),
                        CommandType::MoveTo(MoveToCommand {
                            commandable_entity,
                            grid_tile,
                        }),
                    ]
                }
            };

            commandable.set_queue(
                commands_sequence,
                commandable_entity,
                &mut commands,
                &mut commandable_interrupt_writer,
                &mut tasks_scheduler,
            );
        } else {
            if !config().pawn.wander_when_idle {
                continue;
            }
            if movable.state != MovableState::Idle {
                continue;
            }
            let mut rng = rand::thread_rng();

            let world_pos = transform.translation.truncate();
            let end_tile = find_empty_grid_tile(world_pos, &arc_navmesh.read(), &mut rng, 0);

            commandable.set_queue(
                CommandType::MoveTo(MoveToCommand {
                    commandable_entity,
                    grid_tile: end_tile,
                }),
                commandable_entity,
                &mut commands,
                &mut commandable_interrupt_writer,
                &mut tasks_scheduler,
            );
        }
    }
}
