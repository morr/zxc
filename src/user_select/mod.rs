use crate::*;

use self::structure::Farm;

pub struct UserSelectPlugin;

impl Plugin for UserSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UserSelectionEvent>()
            .init_resource::<UserSelection>()
            .add_systems(Update, select_on_click.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component, Default)]
pub struct UserSelectionMarker;

#[derive(Event, Debug, Default)]
pub struct UserSelectionEvent;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct UserSelection(pub Option<Entity>);

fn select_on_click(
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
