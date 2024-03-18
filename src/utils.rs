use crate::TILE_SIZE;
use bevy::prelude::*;

pub fn render_grid(mut gizmos: Gizmos) {
    let from = -9;
    let to = 9;

    for i in from..to {
        gizmos.line_2d(
            Vec2::new(from as f32 * TILE_SIZE, i as f32 * TILE_SIZE),
            Vec2::new(to as f32 * TILE_SIZE, i as f32 * TILE_SIZE),
            if i == 0 {
                Color::rgb(0.4, 0.4, 0.4)
            } else {
                Color::rgb(0.2, 0.2, 0.2)
            },
        );
        gizmos.line_2d(
            Vec2::new(i as f32 * TILE_SIZE, from as f32 * TILE_SIZE),
            Vec2::new(i as f32 * TILE_SIZE, to as f32 * TILE_SIZE),
            if i == 0 {
                Color::rgb(0.4, 0.4, 0.4)
            } else {
                Color::rgb(0.2, 0.2, 0.2)
            },
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
