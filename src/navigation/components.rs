use bevy::{prelude::*, utils::HashSet};

use crate::{GRID_COLS, GRID_ROWS};

#[derive(Debug, Default)]
pub struct NavTileOccupant {
    pub weight: f32,
    pub occupied_by: HashSet<Entity>,
    pub walkable: bool,
}

#[derive(Resource)]
pub struct Navmesh(pub Vec<Vec<NavTileOccupant>>);

impl Default for Navmesh {
    fn default() -> Self {
        Self(
        (0..GRID_COLS)
            .map(|_| {
                (0..GRID_ROWS)
                    .map(|_| NavTileOccupant::default())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
        )
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
