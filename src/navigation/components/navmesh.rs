use self::navtile::Navtiles;

use super::*;

#[derive(Resource)]
pub struct Navmesh {
    pub navtiles: Navtiles,
    successors: Vec<Vec<Vec<(IVec2, i32)>>>,
}

impl Default for Navmesh {
    fn default() -> Self {
        let navtiles = Navtiles::default();
        let successors = generate_successors(&navtiles);

        Self {
            navtiles,
            successors,
        }
    }
}

fn generate_successors(navtiles: &Navtiles) -> Vec<Vec<Vec<(IVec2, i32)>>> {
    let a = navtiles.0.iter().enumerate().map(|(x, col)| {
        col.iter().enumerate().map(|(y, tile)| {
            tile_successors(
                IVec2 {
                    x: x as i32,
                    y: y as i32,
                },
                &navtiles,
            )
        });
    });

    //             .map(|_| Navtile::default())
    //             .collect::<Vec<Navtile>>()
    //     })
    //     .collect::<Vec<Vec<Navtile>>>();
    Vec::new()
}

fn tile_successors(tile: IVec2, navtiles: &Navtiles) -> Vec<(IVec2, i32)> {
    // [
    //     (x - 1, y),     // left
    //     (x - 1, y - 1), // left-top
    //     (x, y - 1),     // top
    //     (x + 1, y - 1), // top-right
    //     (x + 1, y),     // right
    //     (x + 1, y + 1), // right-bototm
    //     (x, y + 1),     // bottom
    //     (x - 1, y + 1), // bottom-left
    // ]
    // .iter()
    // .filter_map(|&(nx, ny)| {
    //     navmesh.get_if_passable(nx, ny).and_then(|navtile| {
    //         let is_diagonal_movement = x != nx && y != ny;
    //
    //         if !is_diagonal_movement
    //                             // check that both adjacent tiles are passable
    //                             || (navmesh.get_if_passable(x, ny).is_some()
    //                                 && navmesh.get_if_passable(nx, y).is_some())
    //         {
    //             Some((
    //                 IVec2 { x: nx, y: ny },
    //                 if is_diagonal_movement {
    //                     // this is not strictly correct calculation
    //                     // instead of cost * sqrt(2) it should be
    //                     // (tile1.cost + sqrt(2))/2 + (tile2.cost + sqrt(2))/2
    //                     (navtile.cost as f32 * f32::sqrt(2.0)).floor() as i32
    //                 } else {
    //                     navtile.cost
    //                 },
    //             ))
    //         } else {
    //             None
    //         }
    //     })
    // })
    // .collect::<Vec<_>>()
    Vec::new().into()
}
