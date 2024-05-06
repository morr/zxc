use bevy::ecs::system::EntityCommands;

use super::*;

macro_rules! farm_states {
    (
        $(
            (
                $name:ident
                $(,
                    struct $turple_name: ident {
                        $(
                            $field: ident: $ty: ty
                        ),* $(,)?
                    }

                    , $match_field:ident
                )?
            )),* $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Reflect)]
        pub enum FarmState {
            $($name $(($turple_name))? ),*
        }

        $($(
            #[derive(Clone, Eq, PartialEq, Reflect, Debug)]
            pub struct $turple_name {
            $(
                pub $field: $ty
            ),*
            }
        )?)*


        pub mod farm_state {
            use bevy::{prelude::*};

            $(
                #[derive(Component)]
                pub struct $name;
            )*
        }

        impl Farm {
            pub fn change_state(
                &mut self,
                new_state: FarmState,
                entity: Entity,
                commands: &mut Commands,
                state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<FarmState>>,
            ) {
                // println!("FarmState {:?}=>{:?}", self.state, new_state);

                match &self.state {
                    $(FarmState::$name $( ($match_field) )? => {
                        commands.entity(entity).remove::<farm_state::$name>();
                    },)*
                }

                self.state = new_state;

                match &self.state {
                    $(FarmState::$name $( ($match_field) )? => {
                        commands.entity(entity).insert(farm_state::$name);
                    },)*
                }

                state_change_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
            }
        }
    };
}

farm_states!(
    (NotPlanted),
    (
        Planted,
        struct PlantedState {
            growth_timer: Timer,
            tending_rest_timer: Timer,
            tending_rest_started_day: u32,
            is_tending_pending_for_next_day: bool
        },
        _p
    ),
    (Grown),
    (
        Harvested,
        struct HarvestedState {
            rest_timer: Timer
        },
        _h

    ),
);

#[derive(Debug, Component, Reflect)]
pub struct Farm {
    pub state: FarmState,
    pub tendings_done: u32,
}

impl Default for Farm {
    fn default() -> Self {
        Self {
            state: FarmState::NotPlanted,
            tendings_done: 0,
        }
    }
}

impl Farm {
    pub fn progress_state(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        grid_tile: IVec2,
        simulation_day: u32,
        assets: &Res<FarmAssets>,
        state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<FarmState>>,
    ) {
        let new_state = match &self.state {
            FarmState::NotPlanted => FarmState::Planted(PlantedState {
                growth_timer: Timer::from_seconds(
                    days_to_seconds(CONFIG.farming.growth_days),
                    TimerMode::Once,
                ),
                tending_rest_timer: Self::new_tending_rest_timer(),
                tending_rest_started_day: simulation_day,
                is_tending_pending_for_next_day: false
            }),
            FarmState::Planted(_) => FarmState::Grown,
            FarmState::Grown => FarmState::Harvested(HarvestedState {
                rest_timer: Timer::from_seconds(
                    days_to_seconds(CONFIG.farming.harvested_rest_days),
                    TimerMode::Once,
                )
            }),
            FarmState::Harvested(_) => FarmState::NotPlanted,
        };
        // println!("progress_state {:?} => {:?}", self.state, new_state);
        self.change_state(new_state, entity, commands, state_change_event_writer);

        if let FarmState::NotPlanted = self.state {
            self.tendings_done = 0;
        }

        Self::sync_sprite_bundle(grid_tile, &self.state, &mut commands.entity(entity), assets);
        Self::sync_workable(&self.state, &mut commands.entity(entity));
    }

    pub fn yield_amount(&self) -> u32 {
        let basic_yield = CONFIG.farming.basic_yield_percent * CONFIG.farming.max_yield;
        let rest_yield = CONFIG.farming.max_yield - basic_yield;

        let max_tendings = CONFIG.farming.growth_days; // 1 tending per day
        let tendings_percent = if self.tendings_done == 0 {
            0.0
        } else {
            (self.tendings_done as f32).min(max_tendings) / max_tendings
        };

        // println!(
        //     "basic_yield:{} rest_yield:{} tendings_percent:{}",
        //     basic_yield, rest_yield, tendings_percent
        // );
        (basic_yield + (rest_yield * tendings_percent)).round() as u32
    }

    pub fn new_tending_rest_timer() -> Timer {
        Timer::from_seconds(
            hours_to_seconds(CONFIG.farming.tending_rest_hours),
            TimerMode::Once,
        )
    }

    pub fn spawn(
        commands: &mut Commands,
        assets: &Res<FarmAssets>,
        arc_navmesh: &mut Navmesh,
        grid_tile: IVec2,
        state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<FarmState>>,
    ) {
        let farm = Self::default();
        let state = farm.state.clone();

        let mut entity_commands = commands.spawn((
            farm,
            farm_state::NotPlanted,
            Name::new("Farm"),
        ));
        Farm::sync_sprite_bundle(grid_tile, &state, &mut entity_commands, assets);
        Farm::sync_workable(&state, &mut entity_commands);

        let entity = entity_commands.id();

        arc_navmesh.update_cost(
            grid_tile.x..grid_tile.x + FARM_TILE_SIZE,
            grid_tile.y..grid_tile.y + FARM_TILE_SIZE,
            Some((3.0 * COST_MULTIPLIER) as i32),
        );
        state_change_event_writer.send(EntityStateChangeEvent(entity, state));
    }

    pub fn sync_sprite_bundle(
        grid_tile: IVec2,
        state: &FarmState,
        entity_commands: &mut EntityCommands,
        assets: &Res<FarmAssets>,
    ) {
        let size = IVec2::new(FARM_TILE_SIZE, FARM_TILE_SIZE);
        let texture = match state {
            FarmState::NotPlanted => assets.not_planted.clone(),
            FarmState::Planted(_) => assets.planted.clone(),
            FarmState::Grown => assets.grown.clone(),
            FarmState::Harvested(_) => assets.harvested.clone(),
        };

        entity_commands.insert(SpriteBundle {
            texture,
            sprite: Sprite {
                custom_size: Some(size.grid_tile_edge_to_world()),
                ..default()
            },
            transform: Transform::from_translation(
                (grid_tile.grid_tile_edge_to_world() + size.grid_tile_edge_to_world() / 2.0)
                    .extend(STRUCTURE_Z_INDEX),
            ),
            ..default()
        });
    }

    pub fn sync_workable(state: &FarmState, entity_commands: &mut EntityCommands) {
        let maybe_work_amount = match state {
            FarmState::NotPlanted => Some(CONFIG.farming.planting_hours),
            FarmState::Planted(_) => Some(CONFIG.farming.tending_hours),
            FarmState::Grown => Some(CONFIG.farming.harvesting_hours),
            FarmState::Harvested(_) => None,
        };

        if let Some(work_amount) = maybe_work_amount {
            entity_commands.insert(Workable::new(hours_to_seconds(work_amount)));
        }
    }
}

#[derive(Event, Debug)]
pub struct FarmProgressEvent(pub Entity);

#[derive(Event, Debug)]
pub struct FarmTendedEvent(pub Entity);
