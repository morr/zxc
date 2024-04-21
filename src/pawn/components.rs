use super::*;

#[derive(Component, Default)]
pub struct Pawn {
    pub status: PawnStatus,
    pub task: Option<Task>,
}

macro_rules! define_pawn_statuses {
    // Match a tuple of tuples, with the first one treated as default
    (($first_enum_name:ident, $first_component_name:ident) $(, ($enum_name:ident, $component_name:ident))*) => {
        #[derive(Debug, Default)] // Use the standard Default derive
        pub enum PawnStatus {
            #[default] // This marks the first variant as the default.
            $first_enum_name,
            $($enum_name),*
        }

        #[derive(Component)]
        pub struct $first_component_name;

        $(
            #[derive(Component)]
            pub struct $component_name;
        )*

        impl Pawn {
            pub fn change_status(
                &mut self,
                commands: &mut Commands,
                entity: Entity,
                new_status: PawnStatus,
            ) {
                println!("PawnStatus {:?}=>{:?}", self.status, new_status);

                // Remove the old status component
                match self.status {
                    PawnStatus::$first_enum_name => {
                        commands.entity(entity).remove::<$first_component_name>();
                    },
                    $(PawnStatus::$enum_name => {
                        commands.entity(entity).remove::<$component_name>();
                    }),*
                }

                // Update the Pawn's status
                self.status = new_status;

                // Add the new status component
                match self.status {
                    PawnStatus::$first_enum_name => {
                        commands.entity(entity).insert($first_component_name);
                    },
                    $(PawnStatus::$enum_name => {
                        commands.entity(entity).insert($component_name);
                    }),*
                }
            }
        }
    };
}

// Use the macro with the new tuple of pairs format
define_pawn_statuses!(
    (Idle, PawnIdle),
    (Moving, PawnMoving),
    (Working, PawnWorking)
);

#[derive(Component)]
pub struct PawnStatusText;
