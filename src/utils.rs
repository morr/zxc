use bevy::prelude::*;

use crate::TILE_SIZE;

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
