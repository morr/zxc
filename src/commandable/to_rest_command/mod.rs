use super::*;

pub struct ToRestCommandPlugin;

impl Plugin for ToRestCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ToRestCommand>()
            .add_systems(Update, execute_command.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Event, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct ToRestCommand {
    pub commandable_entity: Entity,
}

#[allow(clippy::too_many_arguments)]
fn execute_command(
    mut commands: Commands,
    mut bed_query: Query<(Entity, &Transform, &mut Bed)>,
    mut commandable_query: Query<(&mut Pawn, &mut Commandable, &Transform)>,
    mut command_reader: EventReader<ToRestCommand>,
    mut commandable_event_writer: EventWriter<CommandCompleteEvent>,
    mut available_beds: ResMut<AvailableBeds>,
    mut commandable_interrupt_writer: EventWriter<InternalCommandInterruptEvent>,
    mut commandable_release_resources_writer: EventWriter<ReleaseCommandResourcesEvent>,
    arc_navmesh: Res<ArcNavmesh>,
) {
    for ToRestCommand { commandable_entity } in command_reader.read() {
        match commandable_query.get_mut(*commandable_entity) {
            Ok((mut pawn, mut commandable, pawn_transform)) => {
                commandable.complete_executing(
                    *commandable_entity,
                    &mut commands,
                    &mut commandable_event_writer,
                );

                let (grid_tile, is_sleep_in_bed) = if let Some(bed_entity) = pawn.owned_bed {
                    // move to claimed bed
                    let (_entity, bed_transform, _bed) = bed_query.get(bed_entity).unwrap();

                    (
                        bed_transform.translation.truncate().world_pos_to_grid(),
                        true,
                    )
                } else if available_beds.0 > 0 {
                    // or claim fee bed
                    let mut found_bed_tile = None;
                    for (bed_entity, bed_transform, mut bed) in bed_query.iter_mut() {
                        if bed.owner.is_some() {
                            continue;
                        }
                        bed.claim_by(
                            bed_entity,
                            *commandable_entity,
                            &mut pawn,
                            &mut available_beds,
                        );
                        found_bed_tile =
                            Some(bed_transform.translation.truncate().world_pos_to_grid());
                        break;
                    }
                    (found_bed_tile.unwrap(), true)
                } else {
                    // Check if the current pawn location is empty
                    let current_tile = pawn_transform.translation.truncate().world_pos_to_grid();

                    if arc_navmesh
                        .read()
                        .has_occupants_except_of::<Pawn>(current_tile.x, current_tile.y)
                    {
                        // go to random nearest empty place
                        (
                            find_empty_grid_tile(
                                pawn_transform.translation.truncate(),
                                &arc_navmesh.read(),
                                &mut rand::thread_rng(),
                                0,
                            ),
                            false,
                        )
                    } else {
                        (current_tile, false)
                    }
                };

                commandable.set_queue(
                    [
                        CommandType::MoveTo(MoveToCommand {
                            commandable_entity: *commandable_entity,
                            grid_tile,
                        }),
                        CommandType::Sleep(SleepCommand {
                            commandable_entity: *commandable_entity,
                            is_sleep_in_bed,
                        }),
                    ],
                    *commandable_entity,
                    &mut commands,
                    &mut commandable_interrupt_writer,
                    &mut commandable_release_resources_writer,
                );
            }
            Err(err) => {
                warn!("Failed to get query result: {:?}", err);
                continue;
            }
        }
    }
}

// The command is executed and completed within the same system (execute_command),
// so there won't be any interruption to handle.
// fn handle_internal_interrupts(
//     mut commands: Commands,
//     mut event_reader: EventReader<InternalCommandInterruptEvent>,
// ) {
// }
