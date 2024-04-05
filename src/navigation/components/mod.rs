use bevy::tasks::Task;

use super::*;

expose_submodules!(navtile, navmesh);

pub const COST_MULTIPLIER: f32 = 100.0;
const DEFAULT_COST: i32 = 1;

const INITIAL_NAV_COST: i32 = (DEFAULT_COST as f32 * COST_MULTIPLIER) as i32;

#[derive(Debug)]
pub struct PathfindingResult {
    pub path: Option<Vec<IVec2>>,
    pub start_tile: IVec2,
    pub end_tile: IVec2,
}

#[derive(Debug, Component)]
pub struct PathfindingTask(pub Vec<Task<PathfindingResult>>);
// I want to monitor the size of the async queue so I store multiple tasks instead of a single
// task. It is done so becuse inside of spawned tasks I decrement queue_counter and thus
// the task must always be executed. But under heavy load task may not be executed
// at all becuase if we store only single task in  Entity, the old PathfindingTask
// may be overriden be a new PathfindingTask and thus old tasks is never polled, which
// under heavy load may lead to the task being dropped from the queue.

impl PathfindingTask {
    pub fn new(task: Task<PathfindingResult>) -> Self {
        Self(vec![task])
    }

    pub fn push(&mut self, task: Task<PathfindingResult>) {
        self.0.push(task);
    }
}

#[derive(Debug, Event)]
pub struct PathfindRequestEvent {
    pub entity: Entity,
    pub start_tile: IVec2,
    pub end_tile: IVec2,
}

#[derive(Debug, Event)]
pub struct PathfindAnswerEvent {
    pub entity: Entity,
    pub start_tile: IVec2,
    pub end_tile: IVec2,
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
