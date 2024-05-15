use crate::*;

use self::structure::Farm;

pub struct UserSelectPlugin;

impl Plugin for UserSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UserSelectEvent>()
            .add_systems(Update, select_on_click.run_if(in_state(AppState::Playing)));
    }
}

#[derive(Component, Default)]
pub struct UserSelect;

#[derive(Event, Debug)]
pub struct UserSelectEvent;

fn select_on_click(
    mut commands: Commands,
    mut click_event_reader: EventReader<ClickEvent>,
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
                .insert(UserSelect)
                .insert(ShowAabbGizmo {
                    color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
                });

            // if let Ok((pawn, movable)) = pawn_query.get(*target_id) {
            // }
        }

        for target_id in navmesh.get_occupation::<Farm>(grid_tile.x, grid_tile.y) {
            commands
                .entity(*target_id)
                .insert(UserSelect)
                .insert(ShowAabbGizmo {
                    color: Some(Color::rgba(1.0, 1.0, 1.0, 0.25)),
                });

            // if let Ok((farm, workable)) = farm_query.get(*target_id) {
            // }
        }
    }
}
