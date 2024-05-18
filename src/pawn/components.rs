use std::mem;
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
    pub fn get_task(&self) -> &Task {
        match &self.state {
            PawnState::TaskAssigned(task) | PawnState::Working(task) => task,
            _ => panic!("Pawn must be in a task-assigned state"),
        }
    }

    pub fn is_birthday(&self, total_day: u32) -> bool {
        self.birth_year_day == ElapsedTime::total_day_to_year_day(total_day)
    }

    pub fn decrease_lifetime(&mut self, amount: f32) {
        self.lifetime = f32::max(self.lifetime - amount, 0.0);
    }
}

macro_rules! pawn_states {
    (
        $( ($name:ident $(, $turple_type:ty, $match_field:ident)?)),* $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Reflect)]

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
            ) -> PawnState {
                // println!("PawnState {:?}=>{:?}", self.state, new_state);
                // Remove the old state component
                match &self.state {
                    $(PawnState::$name $( ($match_field) )? => {
                        commands.entity(entity).remove::<pawn_state::$name>();
                    },)*
                }

                // Set the new state and put old state into prev_state
                let prev_state = mem::replace(&mut self.state, new_state);

                // Add the new component
                match &self.state {
                    $(PawnState::$name $( ($match_field) )? => {
                        commands.entity(entity).insert(pawn_state::$name);
                    },)*
                }

                state_change_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
                prev_state
            }
        }
    };
}

pawn_states!(
    (Idle),
    // (Moving),
    (TaskAssigned, Task, _a),
    (Working, Task, _b),
    (ExecutingCommandable),
    (Sleeping),
    (Dead),
);

#[derive(Component)]
pub struct PawnStateText;

// #[derive(Event, Debug)]
// pub struct PawnBirthdayEvent(pub Entity);

#[derive(Event, Debug)]
pub struct PawnDeathEvent(pub Entity);
