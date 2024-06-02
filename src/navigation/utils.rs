use super::*;

use rand::Rng;
use rand_distr::{Distribution, UnitCircle};

pub fn navmesh_index_to_grid_tile(value: usize) -> i32 {
    value as i32 - config().grid.half_size
}

pub fn grid_tile_to_navmesh_index(value: i32) -> usize {
    (value + config().grid.half_size) as usize
}

const MAX_ATTEMPTS_TO_FIND_IDLE_WALK_PATH: usize = 30;

pub fn find_valid_end_tile(
    start_pos: Vec2,
    navmesh: &Navmesh,
    rng: &mut impl Rng,
    recursion_depth: usize,
) -> IVec2 {
    let move_vector: Vec2 = UnitCircle.sample(rng).into();
    let tiles_to_move = rng.gen_range(2.0..(5.0 + recursion_depth as f32)) * config().tile.size;
    let end_tile = (start_pos + move_vector * tiles_to_move).world_pos_to_grid();

    if recursion_depth >= MAX_ATTEMPTS_TO_FIND_IDLE_WALK_PATH {
        return end_tile;
    }

    if recursion_depth >= (MAX_ATTEMPTS_TO_FIND_IDLE_WALK_PATH / 3) {
        warn!(
            "Too deep recursion \"{}\" find_valid_end_tile",
            recursion_depth
        );
    }

    if navmesh.is_passable(end_tile.x, end_tile.y) {
        end_tile
    } else {
        let offsets = [
            IVec2::new(-1, -1), // left-top
            IVec2::new(0, -1),  // top
            IVec2::new(1, -1),  // right-top
            IVec2::new(-1, 0),  // left
            IVec2::new(1, 0),   // right
            IVec2::new(-1, 1),  // left-bottom
            IVec2::new(0, 1),   // bottom
            IVec2::new(1, 1),   // right-bottom
        ];

        offsets
            .iter()
            .map(|offset| end_tile + *offset)
            .find(|&tile| navmesh.is_passable(tile.x, tile.y))
            .unwrap_or_else(|| find_valid_end_tile(start_pos, navmesh, rng, recursion_depth + 1))
    }
}
