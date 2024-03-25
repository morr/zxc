use bevy::prelude::*;

use crate::TILE_SIZE;

pub fn tile_pos_to_world(value: i32) -> f32 {
    // value * TILE_SIZE + TILE_SIZE / 2
    value as f32 * TILE_SIZE // - TILE_SIZE / 2.
}

pub fn tile_pos_to_world_aligned(value: i32) -> f32 {
    tile_pos_to_world(value) - TILE_SIZE / 2.0
}

pub fn world_pos_to_tile(value: f32) -> i32 {
    (value / TILE_SIZE).floor() as i32
}

pub fn wold_pos_to_tile_aligned(value: f32) -> f32 {
    tile_pos_to_world_aligned(world_pos_to_tile(value))
}

pub trait WorldTranslationHelper {
    fn world_pos_to_tile(&self) -> IVec2;
}
pub trait TileTranslationHelper {
    fn tile_pos_to_world(&self) -> Vec2;
    fn tile_pos_to_world_aligned(&self) -> Vec2;
}
//
// impl TranslationHelper for Transform {
//     fn world_pos_to_tile(&self) -> Vec2 {
//         Vec2::new(
//             world_pos_to_tile(self.translation.x),
//             world_pos_to_tile(self.translation.y),
//         )
//     }
//
//     fn tile_pos_to_world(&self) -> Vec2 {
//         Vec2::new(
//             tile_pos_to_world(self.translation.x),
//             tile_pos_to_world(self.translation.y),
//         )
//     }
// }
//
// impl TranslationHelper for GlobalTransform {
//     fn world_pos_to_tile(&self) -> Vec2 {
//         Vec2::new(
//             world_pos_to_tile(self.translation().x),
//             world_pos_to_tile(self.translation().y),
//         )
//     }
//
//     fn tile_pos_to_world(&self) -> Vec2 {
//         Vec2::new(
//             tile_pos_to_world(self.translation().x),
//             tile_pos_to_world(self.translation().y),
//         )
//     }
// }
//
// impl TranslationHelper for Vec3 {
//     fn world_pos_to_tile(&self) -> Vec2 {
//         Vec2::new(world_pos_to_tile(self.x), world_pos_to_tile(self.y))
//     }
//
//     fn tile_pos_to_world(&self) -> Vec2 {
//         Vec2::new(tile_pos_to_world(self.x), tile_pos_to_world(self.y))
//     }
// }
//
impl WorldTranslationHelper for Vec2 {
    fn world_pos_to_tile(&self) -> IVec2 {
        IVec2::new(world_pos_to_tile(self.x), world_pos_to_tile(self.y))
    }
}

impl TileTranslationHelper for IVec2 {
    fn tile_pos_to_world(&self) -> Vec2 {
        Vec2::new(tile_pos_to_world(self.x), tile_pos_to_world(self.y))
    }

    fn tile_pos_to_world_aligned(&self) -> Vec2 {
        Vec2::new(tile_pos_to_world_aligned(self.x), tile_pos_to_world_aligned(self.y))
    }
}
