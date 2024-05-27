use super::*;

use std::collections::VecDeque;
use std::vec;

#[derive(Debug, Clone, Reflect)]
pub enum CommandType {
    MoveTo(MoveToCommand),
    Sleep(SleepCommand),
    ToRest(ToRestCommand),
    UserSelection(UserSelectionCommand),
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
    pub executing: Option<CommandType>,
    pub queue: VecDeque<CommandType>,
    pub state: CommandableState,
}

impl Default for Commandable {
    fn default() -> Self {
        Self {
            executing: None,
            queue: VecDeque::default(),
            state: CommandableState::Idle,
        }
    }
}

#[derive(Event, Debug)]
pub struct CommandCompleteEvent(pub Entity);

#[derive(Event, Debug)]
/// event to interrupt command initiated by 3rd party entity
pub struct RemoteInterruptCommandEvent(pub Entity);

#[derive(Event, Debug)]
/// event to interrupt command initiated by Commandable itself
pub struct InterruptCommandEvent(pub CommandType);

impl Commandable {
    pub fn clear_queue(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        commandable_interrupt_writer: &mut EventWriter<InterruptCommandEvent>,
        tasks_scheduler: &mut EventWriter<ScheduleTaskEvent>,
    ) {
        // println!(
        //     ">>clear_queue state={:?} queue={:?} executing={:?}",
        //     self.state, self.queue, self.executing
        // );
        self.drain_queue(commandable_interrupt_writer, tasks_scheduler);
        self.change_state(CommandableState::Idle, entity, commands);
        // println!(
        //     "state={:?} queue={:?} executing={:?}",
        //     self.state, self.queue, self.executing
        // );
    }

    pub fn set_queue<I>(
        &mut self,
        command_or_commands: I,
        entity: Entity,
        commands: &mut Commands,
        commandable_interrupt_writer: &mut EventWriter<InterruptCommandEvent>,
        tasks_scheduler: &mut EventWriter<ScheduleTaskEvent>,
    ) where
        I: IntoIterator<Item = CommandType>,
    {
        // println!(">>set_queue state={:?}", self.state);
        self.drain_queue(commandable_interrupt_writer, tasks_scheduler);

        self.change_state(CommandableState::PendingExecution, entity, commands);
        self.queue = command_or_commands.into_iter().collect();
        // println!(
        //     "state={:?} queue={:?} executing={:?}",
        //     self.state, self.queue, self.executing
        // );
    }

    pub fn extend_queue<I>(
        &mut self,
        command_or_commands: I,
        entity: Entity,
        commands: &mut Commands,
    ) where
        I: IntoIterator<Item = CommandType>,
    {
        // println!(">>extend_queue state={:?}", self.state);
        if self.state == CommandableState::Idle {
            self.change_state(CommandableState::PendingExecution, entity, commands);
        }
        self.queue.extend(command_or_commands);
        // println!(
        //     "state={:?} queue={:?} executing={:?}",
        //     self.state, self.queue, self.executing
        // );
    }

    pub fn start_executing(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
    ) -> Option<CommandType> {
        let maybe_command_type = self.queue.pop_front();

        if let Some(ref command_type) = maybe_command_type {
            self.executing = Some(command_type.clone());
            self.change_state(CommandableState::Executing, entity, commands);
        } else {
            warn!("Commandable.queue is empty {:?}", self);
            self.change_state(CommandableState::Idle, entity, commands);
        }

        maybe_command_type
    }

    // currently there is no difference betweeen complete and aborted command
    pub fn abort_executing(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        commandable_event_writer: &mut EventWriter<CommandCompleteEvent>,
    ) {
        // println!(">>abort_executing");
        self.complete_executing(entity, commands, commandable_event_writer);
    }

    pub fn complete_executing(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        commandable_event_writer: &mut EventWriter<CommandCompleteEvent>,
    ) {
        // println!(
        //     ">>complete_executing state={:?} executing={:?}",
        //     self.state, self.executing
        // );
        // self.clear_executing(entity, commands);
        // println!(
        //     "state={:?} queue={:?} executing={:?}",
        //     self.state, self.queue, self.executing
        // );

        self.executing = None;
        self.change_state(
            if self.queue.is_empty() {
                CommandableState::Idle
            } else {
                CommandableState::PendingExecution
            },
            entity,
            commands,
        );

        if self.state == CommandableState::Idle {
            commandable_event_writer.send(CommandCompleteEvent(entity));
        }
    }

    // pub fn interrupt_executing(&mut self, entity: Entity, commands: &mut Commands) {
    //     if let Some(command) = self.executing.take() {
    //         self.queue.push_front(command);
    //     }
    //
    //     if self.state != CommandableState::Idle && !self.queue.is_empty() {
    //         self.change_state(CommandableState::PendingExecution, entity, commands);
    //     }
    // }

    // fn clear_executing(&mut self, entity: Entity, commands: &mut Commands) {
    //     self.executing = None;
    //     self.change_state(
    //         if self.queue.is_empty() {
    //             CommandableState::Idle
    //         } else {
    //             CommandableState::PendingExecution
    //         },
    //         entity,
    //         commands,
    //     );
    // }

    fn drain_queue(
        &mut self,
        commandable_interrupt_writer: &mut EventWriter<InterruptCommandEvent>,
        tasks_scheduler: &mut EventWriter<ScheduleTaskEvent>,
    ) {
        if let Some(command_type) = self.executing.take() {
            commandable_interrupt_writer.send(InterruptCommandEvent(command_type));
            // match command {
            //     CommandType::MoveTo(move_to_command) => {
            //         if let Ok(mut movable) = commands.get_mut::<Movable>(move_to_command.0) {
            //             // Safely stop MoveToCommand execution
            //             movable.to_idle(move_to_command.0, commands, None);
            //         }
            //     },
            //     CommandType::Sleep(sleep_command) => {
            //         if let Ok(mut pawn) = commands.get_mut::<Pawn>(sleep_command.0) {
            //             // Safely stop SleepCommand execution
            //             pawn.change_state(PawnState::Idle, sleep_command.0, commands);
            //         }
            //     }
            //     CommandType::ToRest(to_rest_command) => {
            //         // ToRestCommand extends the queue with MoveTo and Sleep commands
            //         // No special state change, just ensure restable is reset
            //         if let Ok(mut restable) = commands.get_mut::<Restable>(to_rest_command.0) {
            //             restable.stamina = FULL_STAMINA; // Example of resetting Restable state
            //         }
            //     }
            //     CommandType::UserSelection(_) => {
            //         // Safely stop UserSelectionCommand execution
            //         // Clear the current user selection
            //         self.current_user_selection.0 = None;
            //     }
            //     CommandType::WorkOn(work_on_command) => {
            //         if let Ok(mut workable) =
            //             commands.get_mut::<Workable>(work_on_command.1.workable_entity)
            //         {
            //             // Safely stop WorkOnCommand execution
            //             workable.reset_amount_done();
            //             workable.change_state(
            //                 WorkableState::Idle,
            //                 work_on_command.1.workable_entity,
            //                 commands,
            //             );
            //         }
            //     }
            // }

            // self.queue.push_front(command_type);
        }

        // cleanup queue and maybe do something with its content
        while let Some(command_type) = self.queue.pop_back() {
            #[allow(clippy::single_match)]
            match command_type {
                CommandType::WorkOn(WorkOnCommand {
                    commandable_entity: _,
                    task,
                }) => {
                    tasks_scheduler.send(ScheduleTaskEvent::push_front(task));
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
