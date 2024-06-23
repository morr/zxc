use super::*;

pub struct DropItemCommandPlugin;

impl Plugin for DropItemCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DropItemCommand>().add_systems(
            Update,
            (execute_command, handle_internal_interrupts)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct DropItemCommand {
    pub commandable_entity: Entity,
    pub carryable_entity: Entity,
}

#[allow(clippy::too_many_arguments)]
fn execute_command(
    mut commands: Commands,
    mut command_reader: EventReader<DropItemCommand>,
    mut commandable_query: Query<(&mut Pawn, &mut Commandable, &Transform)>,
    mut carryable_query: Query<&mut Carryable>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
    mut commandable_interrupt_writer: EventWriter<ExternalCommandInterruptEvent>,
    assets_collection: Res<AssetsCollection>,
    meshes_collection: Res<MeshesCollection>,
) {
    for DropItemCommand {
        commandable_entity,
        carryable_entity,
    } in command_reader.read()
    {
        let (mut pawn, mut commandable, commandable_transform) =
            match commandable_query.get_mut(*commandable_entity) {
                Ok((pawn, commandable, transform)) => (pawn, commandable, transform),
                Err(err) => {
                    warn!(
                        "Failed to get query result for commandable_entity {:?}: {:?}",
                        commandable_entity, err
                    );
                    continue;
                }
            };

        let mut carryable = match carryable_query.get_mut(*carryable_entity) {
            Ok(carryable) => carryable,
            Err(err) => {
                warn!(
                    "Failed to get query result for carryable_entity {:?}: {:?}",
                    carryable_entity, err
                );
                interrupt_commandable_commands_queue!(
                    commandable_interrupt_writer,
                    *commandable_entity
                );
                continue;
            }
        };

        let grid_tile = commandable_transform.world_pos_to_grid();

        carryable.drop_from_inventory(
            &mut pawn,
            *carryable_entity,
            grid_tile,
            &mut commands,
            &assets_collection,
            &meshes_collection,
        );

        commandable.complete_executing(
            *commandable_entity,
            &mut commands,
            &mut commandable_event_writer,
        );
    }
}

fn handle_internal_interrupts(
    // mut commands: Commands,
    mut interrupt_reader: EventReader<InternalCommandInterruptEvent>,
    // mut commandable_query: Query<&mut Commandable>,
    // mut workable_query: Query<&mut Workable>,
    // mut tasks_scheduler: EventWriter<ScheduleTaskEvent>,
    // mut work_complete_event_writer: EventWriter<WorkCompleteEvent>,
) {
    for InternalCommandInterruptEvent(interrupted_command_type) in interrupt_reader.read() {
        if let CommandType::DropItem(interrupted_command) = interrupted_command_type {
            error!("{:?}", interrupted_command)
            // let TaskKind::Work {
            //     workable_entity, ..
            // } = interrupted_command.task.kind
            // else {
            //     panic!("Task kind must be TaskKind::Work");
            // };
            //
            // // Handle the workable entity
            // if let Ok(mut workable) = workable_query.get_mut(workable_entity) {
            //     if let WorkableState::BeingWorked(ref worked_command) = workable.state {
            //         if interrupted_command == worked_command {
            //             tasks_scheduler
            //                 .send(ScheduleTaskEvent::push_front(worked_command.task.clone()));
            //             workable.change_state(WorkableState::Idle, workable_entity, &mut commands);
            //         }
            //     }
            // }
        }
    }
}
