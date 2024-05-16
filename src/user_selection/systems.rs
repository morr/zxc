use super::*;

pub fn find_new_selection_on_click(
    mut commands: Commands,
    mut click_event_reader: EventReader<ClickEvent>,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut current_user_selection: ResMut<UserSelection>,
    mut user_selection_event_writer: EventWriter<UserSelectionEvent>,
) {
    for ClickEvent(grid_tile) in click_event_reader.read() {
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
        if entities.is_empty() {
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

        let new_selection =
            maybe_selection_index.map(|selection_index| entities[selection_index].clone());

        // remove aabb from prev selected
        if let Some(UserSelectionData { id, .. }) = current_user_selection.0 {
            commands.entity(id).remove::<ShowAabbGizmo>();
        }
        // add aabb to to selected
        if let Some(UserSelectionData { id, .. }) = new_selection {
            commands.entity(id).insert(ShowAabbGizmo {
                color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
            });
        }

        *current_user_selection = UserSelection(new_selection);
        user_selection_event_writer.send(UserSelectionEvent);
    }
}
