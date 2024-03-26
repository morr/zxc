use super::*;

use crate::{
    utils::grid_tile_edge_to_world, GRID_COLS, GRID_COLS_HALF, GRID_ROWS, GRID_ROWS_HALF, TILE_SIZE,
};

pub struct DebugGridPlugin;
impl Plugin for DebugGridPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugGridState>().add_systems(
            Update,
            render_grid.run_if(in_state(DebugGridState::Visible)),
        );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugGridState {
    #[default]
    Hidden,
    Visible,
}

pub fn render_grid(mut gizmos: Gizmos) {
    for i in -GRID_ROWS_HALF..GRID_ROWS_HALF {
        gizmos.line_2d(
            Vec2::new(
                grid_tile_edge_to_world(-GRID_COLS_HALF),
                grid_tile_edge_to_world(i),
            ),
            Vec2::new(
                grid_tile_edge_to_world(GRID_COLS_HALF),
                grid_tile_edge_to_world(i),
            ),
            Color::rgba(0.2, 0.2, 0.2, 0.5),
        );
    }

    for i in -GRID_COLS_HALF..GRID_COLS_HALF {
        gizmos.line_2d(
            Vec2::new(
                grid_tile_edge_to_world(i),
                grid_tile_edge_to_world(-GRID_ROWS_HALF),
            ),
            Vec2::new(
                grid_tile_edge_to_world(i),
                grid_tile_edge_to_world(GRID_ROWS_HALF),
            ),
            Color::rgba(0.2, 0.2, 0.2, 0.5),
        );
    }

    gizmos.line_2d(
        Vec2::new(0.0, 0.0),
        Vec2::new(GRID_COLS as f32 * TILE_SIZE, 0.0),
        Color::rgb(1.0, 0.0, 0.0),
    );

    gizmos.line_2d(
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, GRID_ROWS as f32 * TILE_SIZE),
        Color::rgb(0.0, 1.0, 0.0),
    );
}
