use super::*;
use std::collections::VecDeque;

pub struct TasksQueuePlugin;

impl Plugin for TasksQueuePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TasksQueue>()
            .add_observer(on_schedule_task);
    }
}

#[derive(Default, Resource)]
pub struct TasksQueue {
    pub tasks: VecDeque<Task>,
}

impl TasksQueue {
    pub fn push_task_front(&mut self, task: Task) {
        self.tasks.push_front(task);
    }

    pub fn push_task_back(&mut self, task: Task) {
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

    pub fn remove_work_tasks(&mut self, entity: Entity, work_kind: WorkKind) {
        self.tasks.retain(|task| {
            !matches!(**task, TaskKind::Work { workable_entity, work_kind: ref wk }
                if workable_entity == entity && wk == &work_kind)
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect, Deref, DerefMut)]
pub struct Task(pub TaskKind);

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum TaskKind {
    Work {
        workable_entity: Entity,
        work_kind: WorkKind,
    },
    CarryItem {
        carryable_entity: Entity,
        destination_grid_tile: IVec2,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum QueuingType {
    PushBack,
    PushFront,
}

#[derive(Event, Debug)]
pub struct ScheduleTaskEvent(pub Task, pub QueuingType);

impl ScheduleTaskEvent {
    pub fn push_front(task: Task) -> Self {
        Self(task, QueuingType::PushFront)
    }

    pub fn push_back(task: Task) -> Self {
        Self(task, QueuingType::PushBack)
    }
}

fn on_schedule_task(event: On<ScheduleTaskEvent>, mut tasks_queue: ResMut<TasksQueue>) {
    let ScheduleTaskEvent(task, queuing_type) = &*event;
    match queuing_type {
        QueuingType::PushBack => tasks_queue.push_task_back(task.clone()),
        QueuingType::PushFront => tasks_queue.push_task_front(task.clone()),
    };
}
