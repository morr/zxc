use std::collections::VecDeque;

use super::*;

#[derive(Component, Debug, Default, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Workable {
    pub state: WorkableState,
    /// in seconds
    pub work_amount_done: f32,
    /// in seconds
    pub work_amount_total: f32,
}

impl Workable {
    pub fn new(work_amount_total: f32) -> Self {
        Self {
            state: WorkableState::Idle,
            work_amount_total,
            work_amount_done: 0.0,
        }
    }
    pub fn perform_work(&mut self, elapsed_time: f32) {
        self.work_amount_done += elapsed_time * CONFIG.pawn.work_force;
    }

    pub fn is_work_complete(&self) -> bool {
        self.work_amount_done >= self.work_amount_total
    }

    pub fn reset_work_amount_done(&mut self) {
        self.work_amount_done = 0.0;
    }
}

macro_rules! workable_states {
    (
        $( ($name:ident, $state_component_name:ident )),* $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Reflect)]
        pub enum WorkableState {
            $($name),*
        }

        pub mod workable_state {
            use bevy::prelude::*;

            $(
                #[derive(Component, Debug, Reflect)]
                pub struct $state_component_name;
            )*
        }

        impl Workable {
            // pub fn change_state(&mut self, new_state: CommandableState) {
            //     self.state = new_state;
            // }

            pub fn change_state(
                &mut self,
                new_state: WorkableState,
                entity: Entity,
                commands: &mut Commands
            ) -> WorkableState {
                use std::mem;

                // println!("WorkableState {:?}=>{:?}", self.state, new_state);

                // Remove the old state component
                match &self.state {
                    $(WorkableState::$name => {
                        commands.entity(entity).remove::<workable_state::$state_component_name>();
                    },)*
                }

                // Set the new state and put old state into prev_state
                let prev_state = mem::replace(&mut self.state, new_state);

                // Add the new component
                match &self.state {
                    $(WorkableState::$name => {
                        commands.entity(entity).insert(workable_state::$state_component_name);
                    },)*
                }

                prev_state
            }
        }
    };
}

workable_states!(
    (Idle, WorkableStateIdleTag),
    (BeingWorked, WorkableStateBeingWorkedTag),
    // (Executing, WorkableStateExecutingTag)
);


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
pub enum TaskKind {
    FarmPlant,
    FarmTending,
    FarmHarvest,
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub struct Task {
    pub entity: Entity,
    pub kind: TaskKind,
    pub grid_tile: IVec2,
}
