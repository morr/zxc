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
        (Entity, &Pawn, &Movable, &mut Commandable, &Transform),
        (
            With<pawn_state::PawnStateIdleTag>,
            // With<commandable_state::CommandableStateIdleTag>,
        ),
    >,
    mut work_queue: ResMut<TasksQueue>,
    mut commandable_interrupt_writer: EventWriter<InternalCommandInterruptEvent>,
    mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
    arc_navmesh: Res<ArcNavmesh>,
) {
    for (commandable_entity, pawn, movable, mut commandable, transform) in &mut query {
        ensure_state!(PawnState::Idle, pawn.state);
        ensure_state!(CommandableState::Idle, commandable.state);

        if let Some(task) = work_queue.get_task() {
            commandable.set_queue(
                [
                    CommandType::MoveTo(MoveToCommand {
                        commandable_entity,
                        grid_tile: task.grid_tile,
                    }),
                    CommandType::WorkOn(WorkOnCommand {
                        commandable_entity,
                        task,
                    }),
                ],
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
            let end_tile = find_valid_end_tile(world_pos, &arc_navmesh.read(), &mut rng, 0);

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
