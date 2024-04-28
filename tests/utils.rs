use zxc::{grid_tile_center_to_world, grid_tile_edge_to_world, world_pos_to_grid, CONFIG};

#[test]
fn grid_tile_edge_to_world_tests() {
    assert_eq!(grid_tile_edge_to_world(0), 0.0);
    assert_eq!(grid_tile_edge_to_world(1), CONFIG.tile.size);
    assert_eq!(grid_tile_edge_to_world(3), CONFIG.tile.size * 3.0);
}

#[test]
fn grid_tile_center_to_world_tests() {
    assert_eq!(grid_tile_center_to_world(0), CONFIG.tile.size / 2.0);
    assert_eq!(grid_tile_center_to_world(1), CONFIG.tile.size + CONFIG.tile.size / 2.0);
    assert_eq!(grid_tile_center_to_world(3), CONFIG.tile.size * 3.0 + CONFIG.tile.size / 2.0);
}

#[test]
fn world_pos_to_grid_tests() {
    assert_eq!(world_pos_to_grid(0.0), 0);
    assert_eq!(world_pos_to_grid(1.0), 0);
    assert_eq!(world_pos_to_grid(CONFIG.tile.size - 1.0), 0);
    assert_eq!(world_pos_to_grid(CONFIG.tile.size), 1);
    assert_eq!(world_pos_to_grid(CONFIG.tile.size + 1.0), 1);
    assert_eq!(world_pos_to_grid(CONFIG.tile.size * 2.0 - 1.0), 1);
    assert_eq!(world_pos_to_grid(CONFIG.tile.size * 2.0), 2);
}
