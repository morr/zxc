use crate::{GRID_COLS, GRID_ROWS, TILE_SIZE};
use bevy::prelude::*;

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
            Vec2::new(tile_pos_to_world(0.0), tile_pos_to_world(i as f32)),
            Vec2::new(
                tile_pos_to_world(GRID_COLS as f32),
                tile_pos_to_world(i as f32),
            ),
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
            Vec2::new(tile_pos_to_world(i as f32), tile_pos_to_world(0.0)),
            Vec2::new(
                tile_pos_to_world(i as f32),
                tile_pos_to_world(GRID_ROWS as f32),
            ),
            color,
        );
    }
}

pub trait TranslationHelper {
    fn world_pos_to_tile(&self) -> Vec2;
    fn tile_pos_to_world(&self) -> Vec2;
}

impl TranslationHelper for Transform {
    fn world_pos_to_tile(&self) -> Vec2 {
        Vec2::new(
            world_pos_to_tile(self.translation.x),
            world_pos_to_tile(self.translation.y),
        )
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(
            tile_pos_to_world(self.translation.x),
            tile_pos_to_world(self.translation.y),
        )
    }
}

impl TranslationHelper for GlobalTransform {
    fn world_pos_to_tile(&self) -> Vec2 {
        Vec2::new(
            world_pos_to_tile(self.translation().x),
            world_pos_to_tile(self.translation().y),
        )
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(
            tile_pos_to_world(self.translation().x),
            tile_pos_to_world(self.translation().y),
        )
    }
}

impl TranslationHelper for Vec3 {
    fn world_pos_to_tile(&self) -> Vec2 {
        Vec2::new(world_pos_to_tile(self.x), world_pos_to_tile(self.y))
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(tile_pos_to_world(self.x), tile_pos_to_world(self.y))
    }
}

impl TranslationHelper for Vec2 {
    fn world_pos_to_tile(&self) -> Vec2 {
        Vec2::new(world_pos_to_tile(self.x), world_pos_to_tile(self.y))
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(tile_pos_to_world(self.x), tile_pos_to_world(self.y))
    }
}

pub fn world_pos_to_tile(value: f32) -> f32 {
    (value / TILE_SIZE).floor()
}

pub fn tile_pos_to_world(value: f32) -> f32 {
    // value * TILE_SIZE + TILE_SIZE / 2
    value * TILE_SIZE
}
