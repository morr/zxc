use super::*;

pub fn find_new_selection_on_click(
    mut click_event_reader: EventReader<ClickEventStage0>,
    mut click_event_writer: EventWriter<ClickEventStage1>,
    arc_navmesh: ResMut<ArcNavmesh>,
    current_user_selection: Res<CurrentUserSelection>,
    mut user_selection_command_writer: EventWriter<UserSelectionCommand>,
) {
    for ClickEventStage0(grid_tile) in click_event_reader.read() {
        let navmesh = arc_navmesh.read();

        let mut entities: Vec<UserSelectionData> = vec![];

        entities.extend(
            navmesh
                .get_occupants::<Movable>(grid_tile.x, grid_tile.y)
                .map(|&id| UserSelectionData {
                    entity: id,
                    kind: UserSelectionKind::Pawn,
                }),
        );
        entities.extend(
            navmesh
                .get_occupants::<Farm>(grid_tile.x, grid_tile.y)
                .map(|&id| UserSelectionData {
                    entity: id,
                    kind: UserSelectionKind::Farm,
                }),
        );

        // send next stage click event if not user selection action is to be performed
        if entities.is_empty() {
            click_event_writer.send(log_event!(ClickEventStage1(*grid_tile)));
            return;
        }

        let maybe_selection_index = if let Some(UserSelectionData {
            entity: current_id, ..
        }) = &current_user_selection.0
        {
            let current_index = entities
                .iter()
                .position(|UserSelectionData { entity: id, .. }| id == current_id);

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

        user_selection_command_writer.send(log_event!(UserSelectionCommand(maybe_new_selection)));
    }
}
