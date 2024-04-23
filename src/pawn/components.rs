use super::*;

#[derive(Component, Default)]
pub struct Pawn {
    pub state: PawnState,
    pub task: Option<Task>,
}

macro_rules! define_pawn_statees {
    // Match a tuple of tuples, with the first one treated as default
    (($first_enum_name:ident, $first_component_name:ident) $(, ($enum_name:ident, $component_name:ident))*) => {
        #[derive(Debug, Clone, Default)] // Use the standard Default derive
        pub enum PawnState {
            #[default] // This marks the first variant as the default.
            $first_enum_name,
            $($enum_name),*
        }

        // impl From<PawnState> for String {
        //     fn from(state: PawnState) -> Self {
        //         format!("{:?}", state)
        //     }
        // }

        // impl PawnState {
        //     pub fn to_string(&self) -> String {
        //         format!("{:?}", self)
        //     }
        // }

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
                println!("PawnState {:?}=>{:?}", self.state, new_state);

                // Remove the old state component
                match self.state {
                    PawnState::$first_enum_name => {
                        commands.entity(entity).remove::<$first_component_name>();
                    },
                    $(PawnState::$enum_name => {
                        commands.entity(entity).remove::<$component_name>();
                    }),*
                }

                // Update the Pawn's state
                self.state = new_state;

                // Add the new state component
                match self.state {
                    PawnState::$first_enum_name => {
                        commands.entity(entity).insert($first_component_name);
                    },
                    $(PawnState::$enum_name => {
                        commands.entity(entity).insert($component_name);
                    }),*
                }

                pawn_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
            }
        }
    };
}

// Use the macro with the new tuple of pairs format
define_pawn_statees!(
    (Idle, PawnIdle),
    // (Moving, PawnMoving),
    (WorkAssigned, PawnWorkAssigned),
    (Working, PawnWorking)
);

#[derive(Component)]
pub struct PawnStateText;

#[derive(Event, Debug)]
pub struct PawnStartWorkingEvent(pub Entity);
