use super::*;

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Workable {
    pub state: WorkableState,
    pub work_kind: Option<WorkKind>,
    /// in seconds
    pub amount_done: f32,
    /// in seconds
    pub amount_total: f32,
}

#[derive(Event, Debug)]
pub struct WorkCompleteEvent {
    pub commandable_entity: Entity,
    pub workable_entity: Entity,
    pub work_kind: WorkKind,
}

impl Workable {
    pub fn new(props: (Option<WorkKind>, f32)) -> Self {
        Self {
            state: WorkableState::Idle,
            work_kind: props.0,
            amount_total: props.1,
            amount_done: 0.0,
        }
    }

    pub fn perform_work(&mut self, elapsed_time: f32) {
        self.amount_done += elapsed_time * config().pawn.work_force;
    }

    pub fn is_work_complete(&self) -> bool {
        self.amount_done >= self.amount_total
    }

    pub fn reset_amount_done(&mut self) {
        self.amount_done = 0.;
    }

    pub fn reset(
        &mut self,
        props: (Option<WorkKind>, f32),
        entity: Entity,
        commands: &mut Commands,
        // commandable_interrupt_writer: &mut EventWriter<InterruptCommandEvent>,
        commandable_interrupt_writer: &mut EventWriter<ExternalCommandInterruptEvent>,
    ) {
        self.work_kind = props.0;
        self.amount_total = props.1;
        self.amount_done = 0.;
        let prev_state = self.change_state(WorkableState::Idle, entity, commands);

        // if let WorkableState::BeingWorked(command) = prev_state {
        //     // println!("reseting workable in WorkableState::BeingWorked state");
        //     commandable_interrupt_writer.write(log_event!(InterruptCommandEvent(CommandType::WorkOn(command))));
        // }
        // println!("workable reset");
        if let WorkableState::BeingWorked(command) = prev_state {
            // println!("{:?}", ExternalCommandInterruptEvent(command.commandable_entity));
            // println!("reseting workable in WorkableState::BeingWorked state");
            commandable_interrupt_writer.write(log_event!(ExternalCommandInterruptEvent(
                command.commandable_entity
            )));
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum WorkKind {
    FarmPlanting,
    FarmTending,
    FarmHarvest,
}

macro_rules! workable_states {
    (
        $( ($name:ident, $state_component_name:ident $(, ( $($tuple_type:ty),* ), ( $($match_field:ident),* ))? )),* $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Reflect)]
        pub enum WorkableState {
            $($name $( ( $($tuple_type),* ) )? ),*
        }

        pub mod workable_state {
            use bevy::prelude::*;

            $(
                #[derive(Component, Debug, Reflect)]
                pub struct $state_component_name;
            )*
        }

        impl Workable {
            pub fn change_state(
                &mut self,
                new_state: WorkableState,
                entity: Entity,
                commands: &mut Commands,
                // state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<WorkableState>>,
            ) -> WorkableState {
                use std::mem;
                log_state_change!("Workable({:?}).state {:?} => {:?}", entity, self.state, new_state);

                self.remove_old_state_component(commands, entity);
                let prev_state = mem::replace(&mut self.state, new_state);
                self.add_new_state_component(commands, entity);
                // state_change_event_writer.write(log_event!(EntityStateChangeEvent(entity, self.state.clone())));

                prev_state
            }

            fn remove_old_state_component(&self, commands: &mut Commands, entity: Entity) {
                match &self.state {
                    $(WorkableState::$name $( ( $($match_field),* ) )? => {
                        commands.entity(entity).remove::<workable_state::$state_component_name>();
                    },)*
                }
            }

            fn add_new_state_component(&self, commands: &mut Commands, entity: Entity) {
                match &self.state {
                    $(WorkableState::$name $( ( $($match_field),* ) )? => {
                        commands.entity(entity).insert(workable_state::$state_component_name);
                    },)*
                }
            }
        }
    };
}

// Example usage
workable_states!(
    (Idle, WorkableStateIdleTag),
    (
        BeingWorked,
        WorkableStateBeingWorkedTag,
        (WorkOnCommand),
        (_a)
    ),
);
