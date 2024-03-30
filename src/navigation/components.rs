use bevy::utils::HashSet;

use super::*;

pub const COST_MULTIPLIER: f32 = 100.0;
const DEFAULT_COST: i32 = 1;

const INITIAL_NAV_COST: i32 = (DEFAULT_COST as f32 * COST_MULTIPLIER) as i32;
const INITIAL_PASSABLE: bool = true;

#[derive(Debug, Event)]
pub struct PathfindRequestEvent {
    pub entity: Entity,
    pub start: IVec2,
    pub end: IVec2,
}

#[derive(Debug, Event)]
pub struct PathfindAnswerEvent {
    pub entity: Entity,
    pub start: IVec2,
    pub end: IVec2,
    pub path: Option<Vec<IVec2>>,
}

// #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
// pub enum PathfindStatusEnum {
//     #[default]
//     Idle,
//     Pathfinding,
//     // Error,
// }

// #[derive(Component)]
// pub struct PathfindStatus(pub PathfindStatusEnum);

#[derive(Debug)]
pub struct Navtile {
    // pub x: i32,
    // pub y: i32,
    pub cost: i32,
    pub occupied_by: HashSet<Entity>,
    pub passable: bool,
}

impl Navtile {
    // fn new(x: i32, y: i32) -> Self {
    fn new() -> Self {
        Self {
            // x,
            // y,
            cost: INITIAL_NAV_COST,
            occupied_by: HashSet::default(),
            passable: INITIAL_PASSABLE,
        }
    }
}

#[derive(Resource)]
pub struct Navmesh {
    tiles: Vec<Vec<Navtile>>,
    successors: Vec<Vec<Vec<(IVec2, i32)>>>,
}

impl Default for Navmesh {
    fn default() -> Self {
        let tiles = (0..GRID_COLS)
            .map(|_x| {
                (0..GRID_ROWS)
                    .map(|_y| Navtile::new())
                    .collect::<Vec<Navtile>>()
            })
            .collect::<Vec<Vec<Navtile>>>();
        let successors = generate_successors(&tiles);

        Self { tiles, successors }
    }
}

impl Navmesh {
    pub fn get(&self, x: i32, y: i32) -> Option<&Navtile> {
        self.tiles
            .get((x + GRID_COLS_HALF) as usize)?
            .get((y + GRID_ROWS_HALF) as usize)
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Navtile> {
        self.tiles
            .get_mut((x + GRID_COLS_HALF) as usize)?
            .get_mut((y + GRID_ROWS_HALF) as usize)
    }

    pub fn get_if_passable(&self, x: i32, y: i32) -> Option<&Navtile> {
        let result = self.get(x, y);

        if result?.passable {
            result
        } else {
            None
        }
    }

    pub fn for_each_tile_mut<F>(&self, mut lambda: F)
    where
        F: FnMut(&Navtile, IVec2),
    {
        for (x, row) in self.tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                lambda(
                    tile,
                    IVec2::new(x as i32 - GRID_COLS_HALF, y as i32 - GRID_ROWS_HALF),
                );
            }
        }
    }
}

fn generate_successors(navmesh: &Vec<Vec<Navtile>>) -> Vec<Vec<Vec<(IVec2, i32)>>> {
    let a = navmesh.iter().enumerate().map(|(x, col)| {
        col.iter().enumerate().map(|(y, tile)| {
            tile_successors(
                IVec2 {
                    x: x as i32,
                    y: y as i32,
                },
                &navmesh,
            )
        });
    });

    //             .map(|_| Navtile::default())
    //             .collect::<Vec<Navtile>>()
    //     })
    //     .collect::<Vec<Vec<Navtile>>>();
    Vec::new()
}

fn tile_successors(tile: IVec2, navmesh: &Vec<Vec<Navtile>>) -> Vec<(IVec2, i32)> {
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
