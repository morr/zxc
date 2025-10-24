use super::*;

pub fn generate() -> Vec<Vec<Tile>> {
    let mut grid = vec![
        vec![
            Tile {
                grid_tile: IVec2::new(0, 0),
                kind: TileKind::Grass,
                height_noise: 0.0,
                humidity_noise: 0.0,
                props_noise: 0.0
            };
            config().grid.size as usize
        ];
        config().grid.size as usize
    ];

    for (x, row) in grid.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            cell.grid_tile.x = navmesh_index_to_grid_tile(x);
            cell.grid_tile.y = navmesh_index_to_grid_tile(y);
        }
    }

    grid
}
