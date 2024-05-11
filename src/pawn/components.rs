use super::*;

#[derive(Debug, Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Pawn {
    pub state: PawnState,
    pub age: u32,
}

impl Default for Pawn {
    fn default() -> Self {
        Self {
            state: PawnState::Idle,
            age: 16
        }
    }
}

impl Pawn {
    pub fn get_task(&self) -> &Task {
        match &self.state {
            PawnState::WorkAssigned(task) | PawnState::Working(task) => task,
            _ => panic!("Pawn must be in a task-assigned state"),
        }
    }
}

macro_rules! pawn_states {
    (
        $( ($name:ident $(, $turple_type:ty, $match_field:ident)?)),* $(,)?
    ) => {
        #[derive(Debug, Clone, Reflect)]
        pub enum PawnState {
            $($name $(($turple_type))? ),*
        }

        pub mod pawn_state {
            use bevy::{prelude::*};

            $(
                #[derive(Component, Reflect)]
                pub struct $name;
            )*
        }

        impl Pawn {
            pub fn change_state(
                &mut self,
                new_state: PawnState,
                entity: Entity,
                commands: &mut Commands,
                state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<PawnState>>,
            ) {
                // println!("PawnState {:?}=>{:?}", self.state, new_state);
                // Remove the old state component
                match &self.state {
                    $(PawnState::$name $( ($match_field) )? => {
                        commands.entity(entity).remove::<pawn_state::$name>();
                    },)*
                }

                // Set the new state
                self.state = new_state;

                // Add the new component
                match &self.state {
                    $(PawnState::$name $( ($match_field) )? => {
                        commands.entity(entity).insert(pawn_state::$name);
                    },)*
                }

                state_change_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
            }
        }
    };
}

pawn_states!(
    (Idle),
    (Moving),
    (WorkAssigned, Task, _a),
    (Working, Task, _b),
);

#[derive(Component)]
pub struct PawnStateText;
