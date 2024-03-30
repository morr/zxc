use super::*;
use pathfinding::directed::astar::astar;

macro_rules! measure_time {
    ($code:block) => {{
        let start_time = std::time::Instant::now();
        let result = { $code };
        let elapsed_time = start_time.elapsed();
        println!("Elapsed time: {:?}", elapsed_time);
        result
    }};
}

pub fn measure_pathfinding(navmesh: Res<Navmesh>) {
    measure_time!({
        // for _ in 0..100 {
            astar_pathfinding(&navmesh, &IVec2 { x: -255, y: -255 }, &IVec2 { x: 255, y: 255 });
        // }
    })
}

pub fn astar_pathfinding(
    navmesh: &Res<Navmesh>,
    tile_start: &IVec2,
    tile_end: &IVec2,
) -> Option<Vec<IVec2>> {
    if navmesh.navtiles.get_if_passable(tile_end.x, tile_end.y).is_some() {
        astar(
            tile_start,
            |&IVec2 { x, y }| {
                [
                    (x - 1, y),     // left
                    (x - 1, y - 1), // left-top
                    (x, y - 1),     // top
                    (x + 1, y - 1), // top-right
                    (x + 1, y),     // right
                    (x + 1, y + 1), // right-bototm
                    (x, y + 1),     // bottom
                    (x - 1, y + 1), // bottom-left
                ]
                .iter()
                .filter_map(|&(nx, ny)| {
                    navmesh.navtiles.get_if_passable(nx, ny).and_then(|navtile| {
                        let is_diagonal_movement = x != nx && y != ny;

                        if !is_diagonal_movement
                                // check that both adjacent tiles are passable
                                || (navmesh.navtiles.get_if_passable(x, ny).is_some()
                                    && navmesh.navtiles.get_if_passable(nx, y).is_some())
                        {
                            Some((
                                IVec2 { x: nx, y: ny },
                                if is_diagonal_movement {
                                    // this is not strictly correct calculation
                                    // instead of cost * sqrt(2) it should be
                                    // (tile1.cost + sqrt(2))/2 + (tile2.cost + sqrt(2))/2
                                    (navtile.cost as f32 * f32::sqrt(2.0)).floor() as i32
                                } else {
                                    navtile.cost
                                },
                            ))
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>()
            },
            // try (distance_x + distance_y) / 3 as it is suggested in docs
            // https://docs.rs/pathfinding/latest/pathfinding/directed/astar/fn.astar.html
            |&pos| {
                let length = (Vec2::new(pos.x as f32, pos.y as f32)
                    - Vec2::new(tile_end.x as f32, tile_end.y as f32))
                .length();

                // println!("{} {}", length, (length * COST_MULTIPLIER) as i32);
                (length * COST_MULTIPLIER) as i32
            },
            |pos| pos == tile_end,
        )
        .map(|(vec, _cost)| vec)
    } else {
        None
    }
}
