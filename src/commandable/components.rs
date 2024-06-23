use super::*;

use std::collections::VecDeque;
use std::vec;

#[derive(Debug, Clone, Reflect)]
pub enum CommandType {
    DropItem(DropItemCommand),
    MoveTo(MoveToCommand),
    Sleep(SleepCommand),
    TakeItem(TakeItemCommand),
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
/// Event to interrupt command initiated by an external entity
pub struct ExternalCommandInterruptEvent(pub Entity);

#[derive(Event, Debug)]
/// Event to interrupt command initiated by the Commandable itself
pub struct InternalCommandInterruptEvent(pub CommandType);

impl Commandable {
    pub fn clear_queue(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        commandable_interrupt_writer: &mut EventWriter<InternalCommandInterruptEvent>,
        tasks_scheduler: &mut EventWriter<ScheduleTaskEvent>,
    ) {
        trace!("Commandable({:?}) clear_queue", entity);

        self.drain_queue(commandable_interrupt_writer, tasks_scheduler);
        self.change_state(CommandableState::Idle, entity, commands);
    }

    pub fn set_queue<I>(
        &mut self,
        command_or_commands: I,
        entity: Entity,
        commands: &mut Commands,
        commandable_interrupt_writer: &mut EventWriter<InternalCommandInterruptEvent>,
        tasks_scheduler: &mut EventWriter<ScheduleTaskEvent>,
    ) where
        I: IntoIterator<Item = CommandType>,
    {
        let new_queue = command_or_commands.into_iter().collect();
        trace!("Commandable({:?}) set_queue {:?}", entity, new_queue);

        self.drain_queue(commandable_interrupt_writer, tasks_scheduler);
        self.queue = new_queue;
        self.change_state(CommandableState::PendingExecution, entity, commands);
    }

    pub fn extend_queue<I>(
        &mut self,
        command_or_commands: I,
        entity: Entity,
        commands: &mut Commands,
    ) where
        I: IntoIterator<Item = CommandType>,
    {
        let additional_queue = command_or_commands.into_iter().collect::<Vec<_>>();
        trace!("Commandable({:?}) extend_queue {:?}", entity, additional_queue);

        self.queue.extend(additional_queue);
        if self.state == CommandableState::Idle {
            self.change_state(CommandableState::PendingExecution, entity, commands);
        }
    }

    pub fn start_executing(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
    ) -> Option<CommandType> {
        let maybe_command_type = self.queue.pop_front();
        trace!(
            "Commandable({:?}) start_executing {:?}",
            entity,
            maybe_command_type
        );

        if let Some(ref command_type) = maybe_command_type {
            self.executing = Some(command_type.clone());
            self.change_state(CommandableState::Executing, entity, commands);
        } else {
            warn!("Commandable.queue is empty {:?}", self);
            self.change_state(CommandableState::Idle, entity, commands);
        }

        maybe_command_type
    }

    pub fn abort_executing(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        commandable_interrupt_writer: &mut EventWriter<InternalCommandInterruptEvent>,
        tasks_scheduler: &mut EventWriter<ScheduleTaskEvent>,
        commandable_event_writer: &mut EventWriter<CommandCompleteEvent>,
    ) {
        trace!(
            "Commandable({:?}) abort_executing {:?}",
            entity,
            self.executing
        );

        if let Some(command_type) = self.executing.take() {
            commandable_interrupt_writer.send(log_event!(InternalCommandInterruptEvent(command_type)));
        }

        self.drain_queue(commandable_interrupt_writer, tasks_scheduler);
        self.change_state(CommandableState::Idle, entity, commands);
        // this sync pawn state
        commandable_event_writer.send(log_event!(CommandCompleteEvent(entity)));
    }

    pub fn complete_executing(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        commandable_event_writer: &mut EventWriter<CommandCompleteEvent>,
    ) {
        trace!(
            "Commandable({:?}) complete_executing {:?}",
            entity,
            self.executing
        );

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
            // this sync pawn state
            commandable_event_writer.send(log_event!(CommandCompleteEvent(entity)));
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
        commandable_interrupt_writer: &mut EventWriter<InternalCommandInterruptEvent>,
        tasks_scheduler: &mut EventWriter<ScheduleTaskEvent>,
    ) {
        if let Some(command_type) = self.executing.take() {
            commandable_interrupt_writer
                .send(log_event!(InternalCommandInterruptEvent(command_type)));
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

        // pub mod commandable_state {
        //     use bevy::prelude::*;
        //
        //     $(
        //         #[derive(Component, Debug, Reflect)]
        //         pub struct $state_component_name;
        //     )*
        // }

        impl Commandable {
            pub fn change_state(
                &mut self,
                new_state: CommandableState,
                entity: Entity,
                _commands: &mut Commands
            ) -> CommandableState {
                use std::mem;
                log_state_change!("Commandable({:?}).state {:?} => {:?} executing={:?} queue={:?}", entity, self.state, new_state, self.executing, self.queue);

                // self.remove_old_state_component(commands, entity);
                let prev_state = mem::replace(&mut self.state, new_state);
                // self.add_new_state_component(commands, entity);
                // state_change_event_writer.send(log_event!(EntityStateChangeEvent(entity, self.state.clone())));

                prev_state
            }

            // fn remove_old_state_component(&self, commands: &mut Commands, entity: Entity) {
            //     // $(
            //     //     commands.entity(entity).remove::<commandable_state::$state_component_name>();
            //     // )*
            //     match &self.state {
            //         $(CommandableState::$name => {
            //             commands.entity(entity).remove::<commandable_state::$state_component_name>();
            //         },)*
            //     }
            // }
            //
            // fn add_new_state_component(&self, commands: &mut Commands, entity: Entity) {
            //     match &self.state {
            //         $(CommandableState::$name => {
            //             commands.entity(entity).insert(commandable_state::$state_component_name);
            //         },)*
            //     }
            // }

        }
    };
}

commandable_states!(
    (Idle, CommandableStateIdleTag),
    (PendingExecution, CommandableStatePendingExecutionTag),
    (Executing, CommandableStateExecutingTag)
);
