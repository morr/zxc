use crate::{
    utils::tile_pos_to_world, GRID_COLS, GRID_COLS_HALF, GRID_ROWS, GRID_ROWS_HALF, TILE_SIZE,
};
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
    for i in (-1 * GRID_ROWS_HALF as i32)..(GRID_ROWS_HALF as i32) {
        gizmos.line_2d(
            Vec2::new(
                tile_pos_to_world(-1.0 * GRID_COLS_HALF),
                tile_pos_to_world(i as f32),
            ),
            Vec2::new(
                tile_pos_to_world(GRID_COLS_HALF),
                tile_pos_to_world(i as f32),
            ),
            Color::rgba(0.2, 0.2, 0.2, 0.5),
        );
    }

    for i in (-1 * GRID_COLS_HALF as i32)..(GRID_COLS_HALF as i32) {
        gizmos.line_2d(
            Vec2::new(
                tile_pos_to_world(i as f32),
                tile_pos_to_world(-1.0 * GRID_ROWS_HALF),
            ),
            Vec2::new(
                tile_pos_to_world(i as f32),
                tile_pos_to_world(GRID_ROWS_HALF),
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
