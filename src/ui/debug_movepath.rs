use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugMovepathState {
    // MainMenu,
    #[default]
    Hidden,
    Visible,
}

pub struct DebugMovepathPlugin;
impl Plugin for DebugMovepathPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugMovepathState>().add_systems(
            Update,
            render_movepath.run_if(in_state(DebugMovepathState::Visible)),
        );
    }
}

pub fn render_movepath(
    query_pawns: Query<(&Pawn, &Transform), With<Pawn>>,
    mut gizmos: Gizmos,
) {
    for (pawn, transform) in &query_pawns {
        if pawn.move_path.is_empty() {
            continue;
        }

        let current_world = transform.translation.truncate();

        // show the pawn's path
        let mut prev_world = transform.translation.truncate();
        for path_target in pawn.move_path.iter() {
            let iter_world = path_target.grid_tile_center_to_world();

            gizmos.line_2d(
                prev_world,
                iter_world,
                Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.25,
                    alpha: 0.25,
                },
            );
            prev_world = iter_world;
        }

        // move pawn
        let move_target_world = pawn.move_path.front().unwrap().grid_tile_center_to_world();

        gizmos.arrow_2d(current_world, move_target_world, Color::RED);
    }
}
