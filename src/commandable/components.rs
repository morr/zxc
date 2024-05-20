use super::*;

use std::collections::VecDeque;
use std::vec;

#[derive(Debug, Clone, Reflect)]
pub enum CommandType {
    UserSelection(UserSelectionCommand),
    ToRest(ToRestCommand),
    MoveTo(MoveToCommand),
    WorkOn(WorkOnCommand),
}

// it is implemented so a single command can be passed into Commandable.schedule
impl IntoIterator for CommandType {
    type Item = CommandType;
    type IntoIter = vec::IntoIter<CommandType>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Commandable {
    pub in_progress: Option<CommandType>,
    pub queue: VecDeque<CommandType>,
    pub state: CommandableState,
}

impl Default for Commandable {
    fn default() -> Self {
        Self {
            in_progress: None,
            queue: VecDeque::default(),
            state: CommandableState::Idle,
        }
    }
}

#[derive(Event, Debug)]
pub struct CommandExecutedEvent(pub Entity);

impl Commandable {
    pub fn set_queue<I>(
        &mut self,
        command_or_commands: I,
        entity: Entity,
        commands: &mut Commands,
        work_queue: &mut ResMut<TasksQueue>,
    ) where
        I: IntoIterator<Item = CommandType>,
    {
        // println!("schedule_execution {:?}", self.pending);
        self.change_state(CommandableState::PendingExecution, entity, commands);
        self.cleanup_queue(work_queue);
        self.queue = command_or_commands.into_iter().collect();
    }

    pub fn extend_queue<I>(
        &mut self,
        command_or_commands: I,
        entity: Entity,
        commands: &mut Commands,
    ) where
        I: IntoIterator<Item = CommandType>,
    {
        // println!("schedule_execution {:?}", self.pending);
        if self.state == CommandableState::Idle {
            self.change_state(CommandableState::PendingExecution, entity, commands);
        }
        self.queue.extend(command_or_commands);
    }

    pub fn complete_in_progress(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        commandable_event_writer: &mut EventWriter<CommandExecutedEvent>,
    ) {
        // println!("complete_execution");
        self.change_state(
            if self.queue.is_empty() {
                CommandableState::Idle
            } else {
                CommandableState::PendingExecution
            },
            entity,
            commands,
        );
        self.in_progress = None;

        if self.state == CommandableState::Idle {
            commandable_event_writer.send(CommandExecutedEvent(entity));
        }
    }

    pub fn cleanup_queue(&mut self, work_queue: &mut ResMut<TasksQueue>) {
        if let Some(command) = self.in_progress.take() {
            self.queue.push_front(command);
        }

        // cleanup queue and maybe do something with its content
        while let Some(command_type) = self.queue.pop_back() {
            #[allow(clippy::single_match)]
            match command_type {
                CommandType::WorkOn(WorkOnCommand(_entity, task)) => {
                    work_queue.push_task_front(task);
                }
                _ => {}
            }
        }
    }
}

macro_rules! commandable_states {
    (
        $( ($name:ident, $state_component_name:ident )),* $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Reflect)]
        pub enum CommandableState {
            $($name),*
        }

        pub mod commandable_state {
            use bevy::prelude::*;

            $(
                #[derive(Component, Debug, Reflect)]
                pub struct $state_component_name;
            )*
        }

        impl Commandable {
            // pub fn change_state(&mut self, new_state: CommandableState) {
            //     self.state = new_state;
            // }

            pub fn change_state(
                &mut self,
                new_state: CommandableState,
                entity: Entity,
                commands: &mut Commands
            ) -> CommandableState {
                use std::mem;

                // println!("CommandableState {:?}=>{:?}", self.state, new_state);

                // Remove the old state component
                match &self.state {
                    $(CommandableState::$name => {
                        commands.entity(entity).remove::<commandable_state::$state_component_name>();
                    },)*
                }

                // Set the new state and put old state into prev_state
                let prev_state = mem::replace(&mut self.state, new_state);

                // Add the new component
                match &self.state {
                    $(CommandableState::$name => {
                        commands.entity(entity).insert(commandable_state::$state_component_name);
                    },)*
                }

                prev_state
            }
        }
    };
}

commandable_states!(
    (Idle, CommandableStateIdleTag),
    (PendingExecution, CommandableStatePendingExecutionTag),
    (Executing, CommandableStateExecutingTag)
);
