use bevy::{prelude::*, utils::HashSet};

use crate::{GRID_COLS, GRID_COLS_HALF, GRID_ROWS, GRID_ROWS_HALF};

pub const COST_MULTIPLIER: f32 = 100.0;
const DEFAULT_COST: i32 = 1;

const INITIAL_NAV_COST: i32 = (DEFAULT_COST as f32 * COST_MULTIPLIER) as i32;
const INITIAL_PASSABLE: bool = true;

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
pub struct Navmesh(pub Vec<Vec<Navtile>>);

impl Default for Navmesh {
    fn default() -> Self {
        Self(
            (0..GRID_COLS)
                .map(|_| {
                    (0..GRID_ROWS)
                        .map(|_| Navtile::default())
                        .collect::<Vec<Navtile>>()
                })
                .collect::<Vec<Vec<Navtile>>>(),
        )
    }
}

impl Navmesh {
    pub fn get(&self, x: i32, y: i32) -> Option<&Navtile> {
        self.0
            .get((x + GRID_COLS_HALF) as usize)?
            .get((y + GRID_ROWS_HALF) as usize)
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut Navtile> {
        self.0
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
        for (x, row) in self.0.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                lambda(
                    tile,
                    IVec2::new(x as i32 - GRID_COLS_HALF, y as i32 - GRID_ROWS_HALF),
                );
            }
        }
    }
}

#[derive(Debug, Event)]
pub struct PathfindRequestEvent {
    pub entity: Entity,
    pub start: IVec2,
    pub end: IVec2,
}

#[derive(Debug, Event)]
pub struct PathfindAnswerEvent {
    pub entity: Entity,
    pub path: Option<Vec<IVec2>>,
}
