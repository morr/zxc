use super::*;

pub mod navtile;
pub mod navmesh;

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
