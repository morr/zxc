use super::*;

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Workable {
    pub state: WorkableState,
    /// in seconds
    pub amount_done: f32,
    /// in seconds
    pub amount_total: f32,
}

impl Workable {
    pub fn new(amount_total: f32) -> Self {
        Self {
            state: WorkableState::Idle,
            amount_total,
            amount_done: 0.0,
        }
    }
    pub fn perform_work(&mut self, elapsed_time: f32) {
        self.amount_done += elapsed_time * CONFIG.pawn.work_force;
    }

    pub fn is_work_complete(&self) -> bool {
        self.amount_done >= self.amount_total
    }

    pub fn reset_amount_done(&mut self) {
        self.amount_done = 0.0;
    }
}

#[derive(Event, Debug)]
pub struct WorkCompleteEvent {
    pub commandable_entity: Entity,
    pub workable_entity: Entity,
}

macro_rules! workable_states {
    (
        $( ($name:ident, $state_component_name:ident $(, $turple_type:ty, $match_field:ident)?)),* $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Reflect)]
        pub enum WorkableState {
            $($name $(($turple_type))? ),*
        }

        pub mod workable_state {
            use bevy::{prelude::*};

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

                // println!("WorkableState {:?}=>{:?}", self.state, new_state);

                // Remove the old state component
                match &self.state {
                    $(WorkableState::$name $( ($match_field) )? => {
                        commands.entity(entity).remove::<workable_state::$state_component_name>();
                    },)*
                }

                // Set the new state and put old state into prev_state
                let prev_state = mem::replace(&mut self.state, new_state);

                // Add the new component
                match &self.state {
                    $(WorkableState::$name $( ($match_field) )? => {
                        commands.entity(entity).insert(workable_state::$state_component_name);
                    },)*
                }

                // state_change_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
                prev_state
            }
        }
    };
}

workable_states!(
    (Idle, WorkableStateIdleTag),
    (BeingWorked, WorkableStateBeingWorkedTag, Entity, _a),
);
