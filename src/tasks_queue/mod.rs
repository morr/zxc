use super::*;
use std::collections::VecDeque;

pub struct TasksQueuePlugin;

impl Plugin for TasksQueuePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TasksQueue>()
            .add_event::<ScheduleTaskEvent>()
            .add_systems(
                FixedUpdate,
                schedule_task.run_if(in_state(AppState::Playing)),
            );
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
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub struct Task {
    pub kind: TaskKind,
    pub grid_tile: IVec2,
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum TaskKind {
    Work { workable_entity: Entity, work_kind: WorkKind },
    CarryItem { carryable_entity: Entity },
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

fn schedule_task(
    mut event_reader: EventReader<ScheduleTaskEvent>,
    mut tasks_queue: ResMut<TasksQueue>,
) {
    for ScheduleTaskEvent(task, queuing_type) in event_reader.read() {
        match queuing_type {
            QueuingType::PushBack => tasks_queue.push_task_back(task.clone()),
            QueuingType::PushFront => tasks_queue.push_task_front(task.clone()),
        };
    }
}
