use super::*;

pub fn navmesh_index_to_grid_tile(value: usize) -> i32 {
    value as i32 - CONFIG.grid.half_size
}

pub fn grid_tile_to_navmesh_index(value: i32) -> usize {
    (value + CONFIG.grid.half_size) as usize
}
