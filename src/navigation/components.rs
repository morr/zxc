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
    pub cost: i32,
    pub occupied_by: HashSet<Entity>,
    pub passable: bool,
}

impl Default for Navtile {
    fn default() -> Self {
        Self {
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
            .map(|_| {
                (0..GRID_ROWS)
                    .map(|_| Navtile::default())
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

fn generate_successors(tiles: &Vec<Vec<Navtile>>) -> Vec<Vec<Vec<(IVec2, i32)>>> {
    let a = tiles.iter().map(|row| {
    });
    //         (0..GRID_ROWS)
    //             .map(|_| Navtile::default())
    //             .collect::<Vec<Navtile>>()
    //     })
    //     .collect::<Vec<Vec<Navtile>>>();
    Vec::new()
}

fn tile_successors(tiles: &Vec<Vec<Navtile>>) -> Vec<(IVec2, i32)> {
    Vec::new()
}
