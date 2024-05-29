use std::ops::RangeInclusive;

use super::*;
use rand::Rng;

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Pawn {
    pub state: PawnState,

    pub age: u32,
    pub birth_year_day: u32,

    /// in seconds
    pub lifetime: f32,
}

#[derive(Component)]
pub struct DyingMarker;

impl Default for Pawn {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let age = rng.gen_range(RangeInclusive::new(
            CONFIG.pawn.spawn_age.0,
            CONFIG.pawn.spawn_age.1,
        ));
        let lifetime = rng.gen_range(RangeInclusive::new(
            CONFIG.pawn.lifetime_span.0 as f32,
            CONFIG.pawn.lifetime_span.1 as f32,
        )) as f32
            * CONFIG.time.year_duration
            - (age as f32 * CONFIG.time.year_duration);

        Self {
            state: PawnState::Idle,
            age,
            birth_year_day: rng.gen_range(0..CONFIG.time.days_in_year),
            lifetime,
        }
    }
}

impl Pawn {
    // pub fn get_task(&self) -> &Task {
    //     match &self.state {
    //         PawnState::TaskAssigned(task) | PawnState::Working(task) => task,
    //         _ => panic!("Pawn must be in a task-assigned state"),
    //     }
    // }

    pub fn is_birthday(&self, total_day: u32) -> bool {
        self.birth_year_day == ElapsedTime::total_day_to_year_day(total_day)
    }

    pub fn decrease_lifetime(&mut self, amount: f32) {
        self.lifetime = f32::max(self.lifetime - amount, 0.0);
    }
}

macro_rules! pawn_states {
    (
        $( ($name:ident, $state_component_name:ident )),* $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Reflect)]
        pub enum PawnState {
            $($name),*
        }

        pub mod pawn_state {
            use bevy::prelude::*;

            $(
                #[derive(Component, Debug, Reflect)]
                pub struct $state_component_name;
            )*
        }

        impl Pawn {
            pub fn change_state(
                &mut self,
                new_state: PawnState,
                entity: Entity,
                commands: &mut Commands
            ) -> PawnState {
                use std::mem;
                log_state_change!("PawnState {:?}=>{:?}", self.state, new_state);

                self.remove_old_state_component(commands, entity);
                let prev_state = mem::replace(&mut self.state, new_state);
                self.add_new_state_component(commands, entity);
                // state_change_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));

                prev_state
            }

            fn remove_old_state_component(&self, commands: &mut Commands, entity: Entity) {
                match &self.state {
                    $(PawnState::$name => {
                        commands.entity(entity).remove::<pawn_state::$state_component_name>();
                    },)*
                }
            }

            fn add_new_state_component(&self, commands: &mut Commands, entity: Entity) {
                match &self.state {
                    $(PawnState::$name => {
                        commands.entity(entity).insert(pawn_state::$state_component_name);
                    },)*
                }
            }

        }
    };
}

pawn_states!(
    (Idle, PawnStateIdleTag),
    (ExecutingCommand, PawnStateExecutingCommandTag),
    (Sleeping, PawnStateSleepingTag),
    (Dead, PawnStateDeadTag),
);

#[derive(Component)]
pub struct PawnStateText;

// #[derive(Event, Debug)]
// pub struct PawnBirthdayEvent(pub Entity);

#[derive(Event, Debug)]
pub struct PawnDeathEvent(pub Entity);
