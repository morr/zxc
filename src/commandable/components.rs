use super::*;

use std::collections::VecDeque;
use std::vec;

#[derive(Debug)]
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
    pub executing_command: Option<CommandType>,
    pub pending_commands: VecDeque<CommandType>,
    pub state: CommandableState,
}

impl Default for Commandable {
    fn default() -> Self {
        Self {
            executing_command: None,
            pending_commands: VecDeque::default(),
            state: CommandableState::Empty,
        }
    }
}

impl Commandable {
    pub fn execute<I>(&mut self, command_or_commands: I, id: Entity, commands: &mut Commands)
    where
        I: IntoIterator<Item = CommandType>,
    {
        // cleanup queue and maybe do something with its content
        // while let Some(command) = self.queue.pop_back() {
        //     match command {
        //         _ => {}
        //     }
        // }

        self.pending_commands = command_or_commands.into_iter().collect();
        self.change_state(CommandableState::PendingCommands, id, commands);
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

                // println!("CommandableState {:?}=>{:?}", self.state, new_state);

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

commandable_states!(Empty, PendingCommands, Executing);
