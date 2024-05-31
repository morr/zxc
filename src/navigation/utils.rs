use super::*;

pub fn navmesh_index_to_grid_tile(value: usize) -> i32 {
    value as i32 - get_config().grid.half_size
}

pub fn grid_tile_to_navmesh_index(value: i32) -> usize {
    (value + get_config().grid.half_size) as usize
}
