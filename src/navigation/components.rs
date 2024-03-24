use bevy::{prelude::*, utils::HashSet};

use crate::{GRID_COLS, GRID_COLS_HALF, GRID_ROWS, GRID_ROWS_HALF};

const DEFAULT_NAV_WEIGHT: f32 = 1.0;
const DEFAULT_PASSABLE: bool = true;

#[derive(Debug)]
pub struct NavTile {
    pub weight: f32,
    pub occupied_by: HashSet<Entity>,
    pub passable: bool,
}

impl Default for NavTile {
    fn default() -> Self {
        Self {
            weight: DEFAULT_NAV_WEIGHT,
            occupied_by: HashSet::default(),
            passable: true,
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
    fn get(&self, x: i32, y: i32) -> Option<&NavTile> {
        // self.0.get((x + GRID_COLS_HALF) as usize).and_then(|row| row.get((y + GRID_ROWS_HALF) as usize))
        self.0
            .get((x + GRID_COLS_HALF) as usize)?
            .get((y + GRID_ROWS_HALF) as usize)
    }

    fn get_if_passable(&self, x: i32, y: i32) -> Option<&NavTile> {
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
    pub path: Option<Vec<Vec2>>,
    pub entity: Entity,
    pub target: IVec2,
}
