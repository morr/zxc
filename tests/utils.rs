use zxc::{utils::{tile_pos_to_world, world_pos_to_tile}, TILE_SIZE};

#[test]
fn transform_tile_pos_to_world_pos() {
    assert_eq!(tile_pos_to_world(0), 0.0);
    assert_eq!(tile_pos_to_world(1), TILE_SIZE);
    assert_eq!(tile_pos_to_world(3), TILE_SIZE * 3.0);
}

#[test]
fn transform_world_pos_to_tile_pos() {
    assert_eq!(world_pos_to_tile(0.0), 0);
    assert_eq!(world_pos_to_tile(1.0), 0);
    assert_eq!(world_pos_to_tile(TILE_SIZE - 1.0), 0);
    assert_eq!(world_pos_to_tile(TILE_SIZE), 1);
    assert_eq!(world_pos_to_tile(TILE_SIZE + 1.0), 1);
    assert_eq!(world_pos_to_tile(TILE_SIZE * 2.0 - 1.0), 1);
    assert_eq!(world_pos_to_tile(TILE_SIZE * 2.0), 2);
}
