use self::structure::Farm;
use super::*;

pub fn select_on_click(
    mut commands: Commands,
    mut click_event_reader: EventReader<ClickEvent>,
    mut user_select_event_weriter: EventWriter<UserSelectionEvent>,
    mut user_selection: ResMut<UserSelection>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    for event in click_event_reader.read() {
        let grid_tile = event.0;
        let navmesh = arc_navmesh.read();

        // for target_id in navmesh.get_occupation::<Tile>(grid_tile.x, grid_tile.y) {
        // }

        for target_id in navmesh.get_occupation::<Movable>(grid_tile.x, grid_tile.y) {
            commands
                .entity(*target_id)
                .insert(UserSelectionMarker)
                .insert(ShowAabbGizmo {
                    color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
                });

            // if let Ok((pawn, movable)) = pawn_query.get(*target_id) {
            // }
        }

        for target_id in navmesh.get_occupation::<Farm>(grid_tile.x, grid_tile.y) {
            commands
                .entity(*target_id)
                .insert(UserSelectionMarker)
                .insert(ShowAabbGizmo {
                    color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
                });

            // if let Ok((farm, workable)) = farm_query.get(*target_id) {
            // }
        }
        user_select_event_weriter.send(UserSelectionEvent);
    }
}
