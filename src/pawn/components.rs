use super::*;

#[derive(Component, Default)]
pub struct Pawn {
    pub state: PawnState,
}

impl Pawn {
    pub fn get_task(&self) -> &Task {
        match &self.state {
            PawnState::WorkAssigned(task) | PawnState::Working(task) => task,
            _ => panic!("Pawn must be in a task-assigned state"),
        }
    }
}

macro_rules! define_pawn_states {
    (
        ($first_enum_name:ident, $first_component_name:ident)
        $( , ($enum_name:ident, $component_name:ident $(, $turple_type:ty, $match_field:ident)?))*
    ) => {
        #[derive(Debug, Clone, Default)]
        pub enum PawnState {
            #[default]
            $first_enum_name,
            $($enum_name $(($turple_type))? ),*
        }

        #[derive(Component)]
        pub struct $first_component_name;

        $(
            #[derive(Component)]
            pub struct $component_name;
        )*

       impl Pawn {
            pub fn change_state(
                &mut self,
                entity: Entity,
                new_state: PawnState,
                commands: &mut Commands,
                pawn_state_event_writer: &mut EventWriter<EntityStateChangeEvent<PawnState>>,
            ) {
                // println!("PawnState {:?}=>{:?}", self.state, new_state);
                // Remove the old state component
                match &self.state {
                    PawnState::$first_enum_name => {
                        commands.entity(entity).remove::<$first_component_name>();
                    },
                    $(PawnState::$enum_name $( ($match_field) )? => {
                        commands.entity(entity).remove::<$component_name>();
                    },)*
                }

                // Set the new state
                self.state = new_state;

                // Add the new component
                match &self.state {
                    PawnState::$first_enum_name => {
                        commands.entity(entity).insert($first_component_name);
                    },
                    $(PawnState::$enum_name $( ($match_field) )? => {
                        commands.entity(entity).insert($component_name);
                    },)*
                }

                pawn_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
            }
        }
    };
}

// Using the macro
define_pawn_states!(
    (Idle, PawnIdle),
    (Moving, PawnMoving),
    (WorkAssigned, PawnWorkAssigned, Task, _wa),
    (Working, PawnWorking, Task, _w)
);

#[derive(Component)]
pub struct PawnStateText;
