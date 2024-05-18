use super::*;

use std::collections::VecDeque;

#[derive(Debug)]
pub enum Command {
    UserSelection(UserSelectionData),
    ToRest(Entity),
    MoveTo(Entity, IVec2),
}

#[derive(Component, Debug)]
pub struct Commandable {
    pub queue: VecDeque<Command>,
    pub state: CommandableState,
}

impl Default for Commandable {
    fn default() -> Self {
        Self {
            queue: VecDeque::default(),
            state: CommandableState::Empty,
        }
    }
}

impl Commandable {
    pub fn schedule(&mut self, queue: Vec<Command>, id: Entity, commands: &mut Commands) {
        // self.queue = queue.into();
        // self.change_state(CommandableState::Scheduled, id, commands);
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

        // pub mod commandable_state {
        //     use bevy::prelude::*;
        //
        //     $(
        //         #[derive(Component, Reflect)]
        //         pub struct $name;
        //     )*
        // }

        impl Commandable {
            pub fn change_state(&mut self, new_state: CommandableState) {
                self.state = new_state;
            }

            // pub fn change_state(
            //     &mut self,
            //     new_state: CommandableState,
            //     entity: Entity,
            //     commands: &mut Commands
            // ) -> CommandableState {
            //     use std::mem;
            //
            //     // println!("CommandableState {:?}=>{:?}", self.state, new_state);
            //
            //     // Remove the old state component
            //     match &self.state {
            //         $(CommandableState::$name => {
            //             commands.entity(entity).remove::<commandable_state::$name>();
            //         },)*
            //     }
            //
            //     // Set the new state and put old state into prev_state
            //     let prev_state = mem::replace(&mut self.state, new_state);
            //
            //     // Add the new component
            //     match &self.state {
            //         $(CommandableState::$name => {
            //             commands.entity(entity).insert(commandable_state::$name);
            //         },)*
            //     }
            //
            //     prev_state
            // }
        }
    };
}

commandable_states!(Empty, Scheduled, Executing);
