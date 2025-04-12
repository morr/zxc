use super::*;

pub fn generate(noise_data: &NoiseData) -> Vec<Vec<Tile>> {
    let mut grid = vec![
        vec![
            Tile {
                grid_tile: IVec2::new(0, 0),
                kind: TileKind::Grass,
                noise_value: 0.0
            };
            config().grid.size as usize
        ];
        config().grid.size as usize
    ];

    for (x, row) in grid.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            cell.grid_tile.x = navmesh_index_to_grid_tile(x);
            cell.grid_tile.y = navmesh_index_to_grid_tile(y);

            let noise_index = y * config().grid.size as usize + x;
            cell.noise_value = noise_data.0[noise_index];
        }
    }

    grid
}
