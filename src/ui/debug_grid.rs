use crate::{utils::tile_pos_to_world, GRID_COLS, GRID_ROWS, TILE_SIZE};
use bevy::prelude::*;

pub struct DebugGridPlugin;
impl Plugin for DebugGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            render_grid.run_if(in_state(DebugGridState::Visible)),
        )
        .init_state::<DebugGridState>();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugGridState {
    // MainMenu,
    #[default]
    Hidden,
    Visible,
}

pub fn render_grid(mut gizmos: Gizmos) {
    let grid_rows_half = GRID_ROWS / 2.0;
    let grid_cols_half = GRID_COLS / 2.0;

    for i in (-1 * grid_rows_half as i32)..(grid_rows_half as i32) {
        gizmos.line_2d(
            Vec2::new(
                tile_pos_to_world(-1.0 * grid_cols_half),
                tile_pos_to_world(i as f32),
            ),
            Vec2::new(
                tile_pos_to_world(grid_cols_half),
                tile_pos_to_world(i as f32),
            ),
            Color::rgba(0.2, 0.2, 0.2, 0.5),
        );
    }

    for i in (-1 * grid_cols_half as i32)..(grid_cols_half as i32) {
        gizmos.line_2d(
            Vec2::new(
                tile_pos_to_world(i as f32),
                tile_pos_to_world(-1.0 * grid_rows_half),
            ),
            Vec2::new(
                tile_pos_to_world(i as f32),
                tile_pos_to_world(grid_rows_half),
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
