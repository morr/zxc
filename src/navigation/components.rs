use bevy::{prelude::*, utils::HashSet};

use crate::{GRID_COLS, GRID_COLS_HALF, GRID_ROWS, GRID_ROWS_HALF};

const INITIAL_NAV_COST: i32 = 1;
const INITIAL_PASSABLE: bool = true;

#[derive(Debug)]
pub struct NavTile {
    pub cost: i32,
    pub occupied_by: HashSet<Entity>,
    pub passable: bool,
}

impl Default for NavTile {
    fn default() -> Self {
        Self {
            cost: INITIAL_NAV_COST,
            occupied_by: HashSet::default(),
            passable: INITIAL_PASSABLE,
        }
    }
}

#[derive(Resource)]
pub struct NavMesh(pub Vec<Vec<NavTile>>);

impl Default for NavMesh {
    fn default() -> Self {
        Self(
            (0..GRID_COLS)
                .map(|_| {
                    (0..GRID_ROWS)
                        .map(|_| NavTile::default())
                        .collect::<Vec<NavTile>>()
                })
                .collect::<Vec<Vec<NavTile>>>(),
        )
    }
}

impl NavMesh {
    pub fn get(&self, x: i32, y: i32) -> Option<&NavTile> {
        // self.0.get((x + GRID_COLS_HALF) as usize).and_then(|row| row.get((y + GRID_ROWS_HALF) as usize))

        self.0
            .get((x + GRID_COLS_HALF) as usize)?
            .get((y + GRID_ROWS_HALF) as usize)
    }

    pub fn get_if_passable(&self, x: i32, y: i32) -> Option<&NavTile> {
        let result = self.get(x, y);

        if result?.passable {
            result
        } else {
            None
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
