use std::ops::RangeInclusive;

use crate::commandable::Commandable;
use crate::events::EntityStateChangeEvent;
use crate::feedable::Feedable;
use crate::movable::Movable;
use crate::prelude::*;
use crate::restable::Restable;
use crate::story_time::total_day_to_year_day;
use bevy_inspector_egui::prelude::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use rand::Rng;

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
#[require(
    pawn_state::PawnStateIdleTag,
    Commandable,
    Restable,
    Feedable,
    Movable = Movable::new(config().pawn.speed * config().tile.size),
    Name = Name::new("Pawn"),
)]
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
        let mut rng = rand::rng();
        let age = rng.random_range(RangeInclusive::new(
            config().pawn.spawn_age.0,
            config().pawn.spawn_age.1,
        ));
        let lifetime = rng.random_range(RangeInclusive::new(
            config().pawn.lifetime_span.0 as f32,
            config().pawn.lifetime_span.1 as f32,
        )) as f32
            * config().time.year_duration
            - (age as f32 * config().time.year_duration);

        Self {
            state: PawnState::Idle,
            age,
            birth_year_day: rng.random_range(1..=config().time.days_in_year),
            lifetime,
        }
    }
}

impl Pawn {
    pub fn is_birthday(&self, total_day: u32) -> bool {
        self.birth_year_day == total_day_to_year_day(total_day)
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
            $crate::exclusive_state_tags!($($state_component_name),*);
        }

        impl Pawn {
            pub fn change_state(
                &mut self,
                new_state: PawnState,
                entity: Entity,
                commands: &mut Commands,
            ) -> PawnState {
                use std::mem;
                log_state_change!("Pawn({:?}).state {:?} => {:?}", entity, self.state, new_state);

                let prev_state = mem::replace(&mut self.state, new_state);
                // the on_add hook of the new tag removes the old one
                self.add_new_state_component(commands, entity);
                commands.trigger(log_event!(EntityStateChangeEvent(entity, self.state.clone())));

                prev_state
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
    (Dead, PawnStateDeadTag),
);

#[derive(Component)]
pub struct PawnStateText;

#[derive(EntityEvent, Debug)]
pub struct PawnDeathEvent {
    pub entity: Entity,
    pub reason: PawnDeathReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum PawnDeathReason {
    OldAge,
    Starvation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_tags_are_exclusive() {
        let mut world = World::new();

        let entity = world.spawn(pawn_state::PawnStateIdleTag).id();
        world
            .entity_mut(entity)
            .insert(pawn_state::PawnStateExecutingCommandTag);
        world.flush();

        assert!(
            !world
                .entity(entity)
                .contains::<pawn_state::PawnStateIdleTag>()
        );
        assert!(
            world
                .entity(entity)
                .contains::<pawn_state::PawnStateExecutingCommandTag>()
        );

        world
            .entity_mut(entity)
            .insert(pawn_state::PawnStateDeadTag);
        world.flush();

        assert!(
            !world
                .entity(entity)
                .contains::<pawn_state::PawnStateExecutingCommandTag>()
        );
        assert!(
            world
                .entity(entity)
                .contains::<pawn_state::PawnStateDeadTag>()
        );
    }
}
