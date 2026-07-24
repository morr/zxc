use super::*;

pub struct ToRestCommandPlugin;

impl Plugin for ToRestCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(execute_command);
    }
}

#[derive(EntityEvent, Debug, Clone, Reflect, PartialEq, Eq)]
pub struct ToRestCommand {
    #[event_target]
    pub commandable_entity: Entity,
}

#[allow(clippy::too_many_arguments)]
fn execute_command(
    event: On<ToRestCommand>,
    mut commands: Commands,
    bed_query: Query<(Entity, &Transform, Has<BedOwnedBy>), With<Bed>>,
    mut commandable_query: Query<(&mut Commandable, &Transform, Option<&OwnedBed>)>,
    available_beds: Res<AvailableBeds>,
    arc_navmesh: Res<ArcNavmesh>,
) {
    let ToRestCommand { commandable_entity } = *event;

    match commandable_query.get_mut(commandable_entity) {
        Ok((mut commandable, pawn_transform, owned_bed)) => {
            commandable.complete_executing(commandable_entity, &mut commands);

            let (grid_tile, is_sleep_in_bed) = if let Some(bed_entity) =
                owned_bed.and_then(OwnedBed::bed)
            {
                // move to claimed bed
                let (_entity, bed_transform, _) = bed_query
                    .get(bed_entity)
                    .expect("to_rest_command: Owned bed query failed");

                (
                    bed_transform.translation.truncate().world_pos_to_grid(),
                    true,
                )
            } else if available_beds.0 > 0 {
                // or claim a free bed
                let mut found_bed_tile = None;
                for (bed_entity, bed_transform, is_claimed) in bed_query.iter() {
                    if is_claimed {
                        continue;
                    }
                    // AvailableBeds is updated by the BedOwnedBy add/remove observers
                    commands
                        .entity(bed_entity)
                        .insert(BedOwnedBy(commandable_entity));
                    found_bed_tile = Some(bed_transform.translation.truncate().world_pos_to_grid());
                    break;
                }
                (
                    found_bed_tile.expect(
                        "to_rest_command: No unclaimed bed found despite available_beds > 0",
                    ),
                    true,
                )
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
                            &mut rand::rng(),
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
                        commandable_entity,
                        grid_tile,
                    }),
                    CommandType::Sleep(SleepCommand {
                        commandable_entity,
                        is_sleep_in_bed,
                    }),
                ],
                commandable_entity,
                &mut commands,
            );
        }
        Err(err) => {
            warn!("Failed to get query result: {:?}", err);
        }
    }
}

// The command is executed and completed within the same system (execute_command),
// so there won't be any interruption to handle.
// fn on_internal_interrupts(
//     event: On<InternalCommandInterruptEvent>,
//     mut commands: Commands,
// ) {
// }
