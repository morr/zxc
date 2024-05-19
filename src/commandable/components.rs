use bevy::ecs::entity;

use super::*;

use std::collections::VecDeque;
use std::vec;

#[derive(Debug, Clone)]
pub enum CommandType {
    UserSelection(UserSelectionCommand),
    ToRest(ToRestCommand),
    MoveTo(MoveToCommand),
}

// it is implemented so a single command can be passed into Commandable.schedule
impl IntoIterator for CommandType {
    type Item = CommandType;
    type IntoIter = vec::IntoIter<CommandType>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self].into_iter()
    }
}

#[derive(Component, Debug)]
pub struct Commandable {
    pub executing: Option<CommandType>,
    pub pending: VecDeque<CommandType>,
    pub state: CommandableState,
}

impl Default for Commandable {
    fn default() -> Self {
        Self {
            executing: None,
            pending: VecDeque::default(),
            state: CommandableState::Idle,
        }
    }
}

#[derive(Event, Debug)]
pub struct CommandExecutedEvent(pub Entity);

impl Commandable {
    pub fn schedule_execution<I>(&mut self, command_or_commands: I, entity: Entity, commands: &mut Commands)
    where
        I: IntoIterator<Item = CommandType>,
    {
        self.cleanup();

        self.pending = command_or_commands.into_iter().collect();
        println!("schedule_execution {:?}", self.pending);
        self.change_state(CommandableState::PendingExecution, entity, commands);
    }

    pub fn complete_execution(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        commandable_event_writer: &mut EventWriter<CommandExecutedEvent>,
    ) {
        println!("complete_execution");
        self.executing = None;

        self.change_state(
            if self.pending.is_empty() {
                CommandableState::Idle
            } else {
                CommandableState::PendingExecution
            },
            entity,
            commands,
        );

        if self.state == CommandableState::Idle {
            commandable_event_writer.send(CommandExecutedEvent(entity));
        }
    }

    pub fn cleanup(&mut self) {
        if let Some(command) = self.executing.take() {
            self.pending.push_front(command);
        }

        // cleanup queue and maybe do something with its content
        while let Some(_command) = self.pending.pop_back() {
            // match command {
            //     // special logic for some of commands will be here later
            //     // for example it will return Task to the tasks queue
            //     _ => {}
            // }
        }
    }
}

macro_rules! commandable_states {
    (
        $($name:ident),*
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Reflect)]
        pub enum CommandableState {
            $($name),*
        }

        pub mod commandable_state {
            use bevy::prelude::*;

            $(
                #[derive(Component, Reflect)]
                pub struct $name;
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

                println!("CommandableState {:?}=>{:?}", self.state, new_state);

                // Remove the old state component
                match &self.state {
                    $(CommandableState::$name => {
                        commands.entity(entity).remove::<commandable_state::$name>();
                    },)*
                }

                // Set the new state and put old state into prev_state
                let prev_state = mem::replace(&mut self.state, new_state);

                // Add the new component
                match &self.state {
                    $(CommandableState::$name => {
                        commands.entity(entity).insert(commandable_state::$name);
                    },)*
                }

                prev_state
            }
        }
    };
}

commandable_states!(Idle, PendingExecution, Executing);
