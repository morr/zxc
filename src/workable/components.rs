use std::collections::VecDeque;

use super::*;

#[derive(Debug, Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Workable {
    /// in seconds
    pub work_amount_done: f32,
    /// in seconds
    pub work_amount_total: f32,
}

impl Workable {
    pub fn new(work_amount_total: f32) -> Self {
        Self {
            work_amount_total,
            work_amount_done: 0.0,
        }
    }
}

impl Workable {
    pub fn perform_work(&mut self, elapsed_time: f32) {
        self.work_amount_done += elapsed_time * CONFIG.pawn.work_force;
    }

    pub fn is_work_complete(&self) -> bool {
        self.work_amount_done >= self.work_amount_total
    }
}

#[derive(Event, Debug)]
pub struct WorkStartEvent {
    pub pawn_entity: Entity,
}

#[derive(Event, Debug)]
pub struct WorkCompleteEvent {
    pub pawn_entity: Entity,
    pub workable_entity: Entity,
}

#[derive(Default, Resource)]
pub struct TasksQueue {
    tasks: VecDeque<Task>,
}

impl TasksQueue {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push_back(task);
    }

    pub fn get_task(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskKind {
    FarmTilePlant,
    // FarmTileHarvest,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub entity: Entity,
    pub kind: TaskKind,
    pub tile: IVec2,
}
