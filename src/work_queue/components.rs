use std::collections::VecDeque;

use super::*;

#[derive(Default, Resource)]
pub struct WorkQueue {
    tasks: VecDeque<Task>,
}

impl WorkQueue {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    pub fn get_task(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum TaskKind {
//     Farming,
// }

#[derive(Debug, Clone)]
pub struct Task {
    pub entity: Entity,
    // pub kind: TaskKind,
    pub tile: IVec2,
}
