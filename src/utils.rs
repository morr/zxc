use crate::{GRID_COLS, GRID_ROWS, TILE_SIZE};
use bevy::prelude::*;

pub fn render_grid(mut gizmos: Gizmos) {
    for i in 0..GRID_ROWS {
        let color = {
            if i == 0 {
                Color::rgb(1.0, 0.0, 0.0)
            } else if i == GRID_ROWS / 2 {
                Color::rgb(1.0, 1.0, 1.0)
            } else {
                Color::rgb(0.2, 0.2, 0.2)
            }
        };

        gizmos.line_2d(
            Vec2::new(0 as f32 * TILE_SIZE, i as f32 * TILE_SIZE),
            Vec2::new(GRID_COLS as f32 * TILE_SIZE, i as f32 * TILE_SIZE),
            color,
        );
    }

    for i in 0..GRID_COLS {
        let color = {
            if i == 0 {
                Color::rgb(0.0, 1.0, 0.0)
            } else if i == GRID_COLS / 2 {
                Color::rgb(1.0, 1.0, 1.0)
            } else {
                Color::rgb(0.2, 0.2, 0.2)
            }
        };

        gizmos.line_2d(
            Vec2::new(i as f32 * TILE_SIZE, 0 as f32 * TILE_SIZE),
            Vec2::new(i as f32 * TILE_SIZE, GRID_ROWS as f32 * TILE_SIZE),
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
            (self.translation.x / TILE_SIZE).floor(),
            (self.translation.y / TILE_SIZE).floor(),
        )
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(
            self.translation.x * TILE_SIZE - TILE_SIZE / 2.,
            self.translation.y * TILE_SIZE - TILE_SIZE / 2.,
        )
    }
}

impl TranslationHelper for GlobalTransform {
    fn world_pos_to_tile(&self) -> Vec2 {
        Vec2::new(
            (self.translation().x / TILE_SIZE).floor(),
            (self.translation().y / TILE_SIZE).floor(),
        )
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(
            self.translation().x * TILE_SIZE - TILE_SIZE / 2.,
            self.translation().y * TILE_SIZE - TILE_SIZE / 2.,
        )
    }
}

impl TranslationHelper for Vec3 {
    fn world_pos_to_tile(&self) -> Vec2 {
        Vec2::new((self.x / TILE_SIZE).floor(), (self.y / TILE_SIZE).floor())
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(
            self.x * TILE_SIZE - TILE_SIZE / 2.,
            self.y * TILE_SIZE - TILE_SIZE / 2.,
        )
    }
}

impl TranslationHelper for Vec2 {
    fn world_pos_to_tile(&self) -> Vec2 {
        Vec2::new((self.x / TILE_SIZE).floor(), (self.y / TILE_SIZE).floor())
    }

    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(
            self.x * TILE_SIZE + TILE_SIZE / 2.,
            self.y * TILE_SIZE + TILE_SIZE / 2.,
        )
    }
}

pub fn world_pos_to_tile(value: f32) -> f32 {
    (value / TILE_SIZE).floor()
}

pub fn tile_pos_to_world(value: f32) -> f32 {
    value * TILE_SIZE + TILE_SIZE / 2.
}
