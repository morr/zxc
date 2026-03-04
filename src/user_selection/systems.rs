use super::*;

pub fn on_click_stage0(
    event: On<ClickStage0Event>,
    mut commands: Commands,
    arc_navmesh: ResMut<ArcNavmesh>,
    current_user_selection: Res<CurrentUserSelection>,
) {
    let ClickStage0Event(grid_tile) = *event;
    let navmesh = arc_navmesh.read();

    let mut entities: Vec<UserSelectionData> = vec![];

    entities.extend(
        navmesh
            .get_occupants::<Pawn>(grid_tile.x, grid_tile.y)
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
        commands.trigger(log_event!(ClickStage1Event(grid_tile)));
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

    commands.trigger(log_event!(UserSelectionCommand(maybe_new_selection)));
}
