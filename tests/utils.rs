use zxc::{utils::{grid_tile_center_to_world, grid_tile_edge_to_world, world_pos_to_grid}, TILE_SIZE};

#[test]
fn transform_tile_pos_edge_to_world_pos() {
    assert_eq!(grid_tile_edge_to_world(0), 0.0);
    assert_eq!(grid_tile_edge_to_world(1), TILE_SIZE);
    assert_eq!(grid_tile_edge_to_world(3), TILE_SIZE * 3.0);
}

#[test]
fn transform_tile_pos_center_to_world_pos() {
    assert_eq!(grid_tile_center_to_world(0), TILE_SIZE / 2.0);
    assert_eq!(grid_tile_center_to_world(1), TILE_SIZE + TILE_SIZE / 2.0);
    assert_eq!(grid_tile_center_to_world(3), TILE_SIZE * 3.0 + TILE_SIZE / 2.0);
}

#[test]
fn transform_world_pos_to_tile_pos() {
    assert_eq!(world_pos_to_grid(0.0), 0);
    assert_eq!(world_pos_to_grid(1.0), 0);
    assert_eq!(world_pos_to_grid(TILE_SIZE - 1.0), 0);
    assert_eq!(world_pos_to_grid(TILE_SIZE), 1);
    assert_eq!(world_pos_to_grid(TILE_SIZE + 1.0), 1);
    assert_eq!(world_pos_to_grid(TILE_SIZE * 2.0 - 1.0), 1);
    assert_eq!(world_pos_to_grid(TILE_SIZE * 2.0), 2);
}
