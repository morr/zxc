use super::*;

// pub fn select_on_click(
//     mut commands: Commands,
//     mut click_event_reader: EventReader<ClickEvent>,
//     mut user_select_event_writer: EventWriter<UserSelectionEvent>,
//     mut user_selection: ResMut<UserSelection>,
//     arc_navmesh: ResMut<ArcNavmesh>,
// ) {
//     for event in click_event_reader.read() {
//         let grid_tile = event.0;
//         let navmesh = arc_navmesh.read();
//
//         // for target_id in navmesh.get_occupation::<Tile>(grid_tile.x, grid_tile.y) {
//         // }
//
//         for target_id in navmesh.get_occupation::<Movable>(grid_tile.x, grid_tile.y) {
//             commands
//                 .entity(*target_id)
//                 .insert(UserSelectionMarker)
//                 .insert(ShowAabbGizmo {
//                     color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
//                 });
//
//             // if let Ok((pawn, movable)) = pawn_query.get(*target_id) {
//             // }
//         }
//
//         for target_id in navmesh.get_occupation::<Farm>(grid_tile.x, grid_tile.y) {
//             commands
//                 .entity(*target_id)
//                 .insert(UserSelectionMarker)
//                 .insert(ShowAabbGizmo {
//                     color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
//                 });
//
//             // if let Ok((farm, workable)) = farm_query.get(*target_id) {
//             // }
//         }
//         user_select_event_weriter.send(UserSelectionEvent);
//     }
// }

pub fn find_new_selection_on_click(
    mut commands: Commands,
    mut click_event_reader: EventReader<ClickEvent>,
    arc_navmesh: ResMut<ArcNavmesh>,
    mut user_selection: ResMut<UserSelection>,
) {
    for ClickEvent(grid_tile) in click_event_reader.read() {
        let navmesh = arc_navmesh.read();

        let mut entities: Vec<UserSelectionData> = vec![];

        entities.extend(
            navmesh
                .get_occupation::<Pawn>(grid_tile.x, grid_tile.y)
                .map(|&id| UserSelectionData {
                    id,
                    kind: SelectionKind::Pawn,
                }),
        );
        entities.extend(
            navmesh
                .get_occupation::<Farm>(grid_tile.x, grid_tile.y)
                .map(|&id| UserSelectionData {
                    id,
                    kind: SelectionKind::Farm,
                }),
        );
        if entities.is_empty() {
            return;
        }

        let maybe_selection_index =
            if let Some(UserSelectionData { id: current_id, .. }) = &user_selection.0 {
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

        perform_selection(new_selection, &mut user_selection, &mut commands);
    }
}

fn perform_selection(
    maybe_selection: Option<UserSelectionData>,
    user_selection: &mut ResMut<UserSelection>,
    commands: &mut Commands,
) {
    // if let Some(UserSelectionData { id, .. }) = user_selection.0 {
    //     commands.entity(id).remove::<ShowAabbGizmo>();
    // }
    // if let Some(UserSelectionData { id, .. }) = maybe_selection {
    //     commands.entity(id).insert(ShowAabbGizmo {
    //         color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
    //     });
    // }

    **user_selection = UserSelection(maybe_selection);
    println!("{:?}", user_selection);
}
