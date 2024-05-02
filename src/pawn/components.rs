use super::*;

#[derive(Component)]
pub struct Pawn {
    pub state: PawnState,
}

impl Default for Pawn {
    fn default() -> Self {
        Self {
            state: PawnState::Idle,
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
        $( ($enum_name:ident, $component_name:ident $(, $turple_type:ty, $match_field:ident)?)),* $(,)?
    ) => {
        #[derive(Debug, Clone)]
        pub enum PawnState {
            $($enum_name $(($turple_type))? ),*
        }

        $(
            #[derive(Component)]
            pub struct $component_name;
        )*

       impl Pawn {
            pub fn change_state(
                &mut self,
                new_state: PawnState,
                entity: Entity,
                commands: &mut Commands,
                pawn_state_event_writer: &mut EventWriter<EntityStateChangeEvent<PawnState>>,
            ) {
                // println!("PawnState {:?}=>{:?}", self.state, new_state);
                // Remove the old state component
                match &self.state {
                    $(PawnState::$enum_name $( ($match_field) )? => {
                        commands.entity(entity).remove::<$component_name>();
                    },)*
                }

                // Set the new state
                self.state = new_state;

                // Add the new component
                match &self.state {
                    $(PawnState::$enum_name $( ($match_field) )? => {
                        commands.entity(entity).insert($component_name);
                    },)*
                }

                pawn_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
            }
        }
    };
}

pawn_states!(
    (Idle, PawnIdle),
    (Moving, PawnMoving),
    (WorkAssigned, PawnWorkAssigned, Task, _a),
    (Working, PawnWorking, Task, _b),
);

#[derive(Component)]
pub struct PawnStateText;
