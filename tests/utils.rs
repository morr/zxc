mod apply_global_config;

mod utils {
    use zxc::*;

    #[test]
    fn grid_tile_edge_to_world_tests() {
        assert_eq!(grid_tile_edge_to_world(0), 0.0);
        assert_eq!(grid_tile_edge_to_world(1), config().tile.size);
        assert_eq!(grid_tile_edge_to_world(3), config().tile.size * 3.0);
    }

    #[test]
    fn grid_tile_center_to_world_tests() {
        assert_eq!(grid_tile_center_to_world(0), config().tile.size / 2.0);
        assert_eq!(
            grid_tile_center_to_world(1),
            config().tile.size + config().tile.size / 2.0
        );
        assert_eq!(
            grid_tile_center_to_world(3),
            config().tile.size * 3.0 + config().tile.size / 2.0
        );
    }

    #[test]
    fn world_pos_to_grid_tests() {
        assert_eq!(world_pos_to_grid(0.0), 0);
        assert_eq!(world_pos_to_grid(1.0), 0);
        assert_eq!(world_pos_to_grid(config().tile.size - 1.0), 0);
        assert_eq!(world_pos_to_grid(config().tile.size), 1);
        assert_eq!(world_pos_to_grid(config().tile.size + 1.0), 1);
        assert_eq!(world_pos_to_grid(config().tile.size * 2.0 - 1.0), 1);
        assert_eq!(world_pos_to_grid(config().tile.size * 2.0), 2);
    }

    #[test]
    fn hours_to_seconds_tests() {
        assert_eq!(hours_to_seconds(1.0), config().time.day_duration / 24.0);
    }
}
