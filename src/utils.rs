use bevy::prelude::*;

use crate::TILE_SIZE;

pub fn grid_tile_edge_to_world(value: i32) -> f32 {
    // value as f32 * TILE_SIZE - TILE_SIZE / 2.0
    value as f32 * TILE_SIZE
}

pub fn grid_tile_center_to_world(value: i32) -> f32 {
    grid_tile_edge_to_world(value) + TILE_SIZE / 2.0
}

// pub fn tile_pos_to_world_aligned(value: i32) -> f32 {
//     tile_pos_to_world(value) + TILE_SIZE / 2.0
// }

pub fn world_pos_to_grid(value: f32) -> i32 {
    (value / TILE_SIZE).floor() as i32
}

// pub fn world_pos_to_tile_aligned(value: f32) -> f32 {
//     tile_pos_to_world_aligned(world_pos_to_tile(value))
// }

pub trait WorldTranslationHelper {
    fn world_pos_to_grid(&self) -> IVec2;
}
pub trait GridTranslationHelper {
    fn grid_tile_edge_to_world(&self) -> Vec2;
    fn grid_tile_center_to_world(&self) -> Vec2;
}

impl WorldTranslationHelper for Vec2 {
    fn world_pos_to_grid(&self) -> IVec2 {
        IVec2::new(world_pos_to_grid(self.x), world_pos_to_grid(self.y))
    }
}

impl GridTranslationHelper for IVec2 {
    fn grid_tile_edge_to_world(&self) -> Vec2 {
        Vec2::new(grid_tile_edge_to_world(self.x), grid_tile_edge_to_world(self.y))
    }

    fn grid_tile_center_to_world(&self) -> Vec2 {
        Vec2::new(grid_tile_center_to_world(self.x), grid_tile_center_to_world(self.y))
    }
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
