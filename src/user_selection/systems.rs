use super::*;

pub fn find_new_selection_on_click(
    mut click_event_reader: EventReader<ClickEventStage0>,
    mut click_event_writer: EventWriter<ClickEventStage1>,
    arc_navmesh: ResMut<ArcNavmesh>,
    current_user_selection: Res<UserSelection>,
    mut user_selection_command_writer: EventWriter<UserSelectionCommand>,
) {
    for ClickEventStage0(grid_tile) in click_event_reader.read() {
        let navmesh = arc_navmesh.read();

        let mut entities: Vec<UserSelectionData> = vec![];

        entities.extend(
            navmesh
                .get_occupation::<Movable>(grid_tile.x, grid_tile.y)
                .map(|&id| UserSelectionData {
                    id,
                    kind: UserSelectionKind::Pawn,
                }),
        );
        entities.extend(
            navmesh
                .get_occupation::<Farm>(grid_tile.x, grid_tile.y)
                .map(|&id| UserSelectionData {
                    id,
                    kind: UserSelectionKind::Farm,
                }),
        );

        // send next stage click event if not user selection action is to be performed
        if entities.is_empty() {
            click_event_writer.send(ClickEventStage1(*grid_tile));
            return;
        }

        let maybe_selection_index =
            if let Some(UserSelectionData { id: current_id, .. }) = &current_user_selection.0 {
                let current_index = entities
                    .iter()
                    .position(|UserSelectionData { id, .. }| id == current_id);

                match current_index {
                    Some(index) => {
                        if entities.len() == 1 {
                            None
                        } else {
                            Some((index + 1) % entities.len())
                        }
                    }
                    None => Some(0),
                }
            } else {
                Some(0)
            };

        let maybe_new_selection =
            maybe_selection_index.map(|selection_index| entities[selection_index].clone());

        user_selection_command_writer.send(UserSelectionCommand(maybe_new_selection));
    }
}

pub fn apply_user_selection_command(
    mut commands: Commands,
    mut current_user_selection: ResMut<UserSelection>,
    mut user_selection_command_reader: EventReader<UserSelectionCommand>,
    mut user_selection_change_event_writer: EventWriter<UserSelectionChangeEvent>,
) {
    for UserSelectionCommand(maybe_new_selection) in user_selection_command_reader.read() {
        // remove aabb from prev selected
        if let Some(UserSelectionData { id, .. }) = current_user_selection.0 {
            commands.entity(id).remove::<ShowAabbGizmo>();
        }
        // add aabb to to selected
        if let Some(UserSelectionData { id, .. }) = maybe_new_selection {
            commands.entity(*id).insert(ShowAabbGizmo {
                color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
            });
        }

        *current_user_selection = UserSelection(maybe_new_selection.clone());
        user_selection_change_event_writer.send(UserSelectionChangeEvent);
    }
}
