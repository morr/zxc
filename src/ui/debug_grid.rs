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

    for i in 1..GRID_ROWS {
        let color = {
            if i == GRID_ROWS / 2 {
                Color::rgb(1.0, 1.0, 1.0)
            } else {
                Color::rgb(0.2, 0.2, 0.2)
            }
        };

        gizmos.line_2d(
            Vec2::new(tile_pos_to_world(0), tile_pos_to_world(i)),
            Vec2::new(tile_pos_to_world(GRID_COLS), tile_pos_to_world(i)),
            color,
        );
    }

    for i in 1..GRID_COLS {
        let color = {
            if i == GRID_COLS / 2 {
                Color::rgb(1.0, 1.0, 1.0)
            } else {
                Color::rgb(0.2, 0.2, 0.2)
            }
        };

        gizmos.line_2d(
            Vec2::new(tile_pos_to_world(i), tile_pos_to_world(0)),
            Vec2::new(tile_pos_to_world(i), tile_pos_to_world(GRID_ROWS)),
            color,
        );
    }
}
