use super::*;

pub fn grid_tile_edge_to_world(value: i32) -> f32 {
    value as f32 * config().tile.size
}

pub fn grid_tile_center_to_world(value: i32) -> f32 {
    grid_tile_edge_to_world(value) + config().tile.size / 2.0
}

pub fn world_pos_to_grid(value: f32) -> i32 {
    (value / config().tile.size).floor() as i32
}

pub fn sprite_size(width: f32, aspectratio: f32) -> Vec2 {
    Vec2::new(width, sprite_height(width, aspectratio))
}

pub fn sprite_height(width: f32, aspectratio: f32) -> f32 {
    width / aspectratio
}

pub fn sprite_transform(grid_tile: IVec2, sprite_size: Vec2, z_index: f32) -> Transform {
    Transform::from_xyz(
        grid_tile_edge_to_world(grid_tile.x) + sprite_size.x / 2.,
        grid_tile_edge_to_world(grid_tile.y) + sprite_size.y / 2.,
        z_index,
    )
}

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
        Vec2::new(
            grid_tile_edge_to_world(self.x),
            grid_tile_edge_to_world(self.y),
        )
    }

    fn grid_tile_center_to_world(&self) -> Vec2 {
        Vec2::new(
            grid_tile_center_to_world(self.x),
            grid_tile_center_to_world(self.y),
        )
    }
}

impl WorldTranslationHelper for Transform {
    fn world_pos_to_grid(&self) -> IVec2 {
        self.translation.truncate().world_pos_to_grid()
    }
}

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
