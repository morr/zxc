use super::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugMovepathState {
    Hidden,
    #[default]
    Visible,
}

pub struct DebugMovepathPlugin;
impl Plugin for DebugMovepathPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugMovepathState>().add_systems(
            Update,
            render_movepath
                .run_if(in_state(AppState::Playing))
                .run_if(in_state(DebugMovepathState::Visible)),
        );
    }
}

pub fn render_movepath(
    query_pawns: Query<(&Movable, &Transform), With<Movable>>,
    mut gizmos: Gizmos,
) {
    for (movable, transform) in &query_pawns {
        if movable.path.is_empty() {
            continue;
        }

        let color = Color::srgba(1.0, 1.0, 0.25, 0.25);

        let mut prev_world = transform.translation.truncate();
        for (index, path_target) in movable.path.iter().enumerate() {
            let iter_world = path_target.grid_tile_center_to_world();

            if index < movable.path.len() - 1 {
                gizmos.line_2d(prev_world, iter_world, color);
            } else {
                gizmos.arrow_2d(prev_world, iter_world, color);
            }
            prev_world = iter_world;
        }
    }
}
