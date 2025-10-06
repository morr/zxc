use super::*;
use bevy::ecs::system::EntityCommands;

macro_rules! farm_states {
    (
        $(
            (
                $name:ident
                $(,
                    struct $state_struct: ident {
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
            $($name $(($state_struct))? ),*
        }

        $($(
            #[derive(Clone, Eq, PartialEq, Reflect, Debug)]
            pub struct $state_struct {
                $(
                    pub $field: $ty
                ),*
            }
        )?)*

        pub mod farm_state {
            use bevy::prelude::*;

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
                state_change_event_writer: &mut MessageWriter<EntityStateChangeMessage<FarmState>>,
            ) {
                self.remove_old_state_component(commands, entity);
                self.state = new_state;
                self.add_new_state_component(commands, entity);
                state_change_event_writer.write(log_event!(EntityStateChangeMessage(entity, self.state.clone())));
            }
        }

        impl Farm {
            fn remove_old_state_component(&self, commands: &mut Commands, entity: Entity) {
                match &self.state {
                    $(FarmState::$name $( ($match_field) )? => {
                        commands.entity(entity).remove::<farm_state::$name>();
                    },)*
                }
            }

            fn add_new_state_component(&self, commands: &mut Commands, entity: Entity) {
                match &self.state {
                    $(FarmState::$name $( ($match_field) )? => {
                        commands.entity(entity).insert(farm_state::$name);
                    },)*
                }
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

#[derive(Component, Debug, Reflect)]
#[require(Name)]
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
    pub fn spawn(
        grid_tile: IVec2,
        commands: &mut Commands,
        assets: &Res<FarmAssets>,
        navmesh: &mut Navmesh,
        state_change_event_writer: &mut MessageWriter<EntityStateChangeMessage<FarmState>>,
    ) {
        let farm = Self::default();
        let farm_state = farm.state.clone(); // Clone the state here
        let workable = Workable::new(workable_props(&farm.state));

        let mut entity_commands =
            commands.spawn((farm, farm_state::NotPlanted, Name::new("Farm"), workable));

        Farm::sync_sprite_bundle(grid_tile, &farm_state, &mut entity_commands, assets); // Use the cloned state

        let entity = entity_commands.id();

        navmesh.update_cost(
            grid_tile.x..grid_tile.x + FARM_TILE_SIZE,
            grid_tile.y..grid_tile.y + FARM_TILE_SIZE,
            Navtile::config_cost_to_pathfinding_cost(config().movement_cost.farm),
        );
        navmesh.add_occupant::<Farm>(&entity, grid_tile.x, grid_tile.y);

        state_change_event_writer.write(log_event!(EntityStateChangeMessage(entity, farm_state)));
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

        entity_commands.insert((
            Sprite {
                image: texture,
                custom_size: Some(size.grid_tile_edge_to_world()),
                ..default()
            },
            Transform::from_translation(
                (grid_tile.grid_tile_edge_to_world() + size.grid_tile_edge_to_world() / 2.0)
                    .extend(STRUCTURE_Z_INDEX),
            ),
        ));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn progress_state(
        &mut self,
        entity: Entity,
        workable: &mut Workable,
        commands: &mut Commands,
        grid_tile: IVec2,
        simulation_day: u32,
        assets: &Res<FarmAssets>,
        state_change_event_writer: &mut MessageWriter<EntityStateChangeMessage<FarmState>>,
        // commandable_interrupt_writer: &mut MessageWriter<InterruptCommandEvent>,
        commandable_interrupt_writer: &mut MessageWriter<ExternalCommandInterruptMessage>,
    ) {
        let new_state = match &self.state {
            FarmState::NotPlanted => FarmState::Planted(PlantedState {
                growth_timer: Timer::from_seconds(
                    days_to_seconds(config().farming.growth_days),
                    TimerMode::Once,
                ),
                tending_rest_timer: Self::new_tending_rest_timer(),
                tending_rest_started_day: simulation_day,
                is_tending_pending_for_next_day: false,
            }),
            FarmState::Planted(_) => FarmState::Grown,
            FarmState::Grown => FarmState::Harvested(HarvestedState {
                rest_timer: Timer::from_seconds(
                    days_to_seconds(config().farming.harvested_rest_days),
                    TimerMode::Once,
                ),
            }),
            FarmState::Harvested(_) => FarmState::NotPlanted,
        };

        self.change_state(new_state, entity, commands, state_change_event_writer);

        if let FarmState::NotPlanted = self.state {
            self.tendings_done = 0;
        }

        workable.reset(
            workable_props(&self.state),
            entity,
            commands,
            commandable_interrupt_writer,
        );
        Self::sync_sprite_bundle(grid_tile, &self.state, &mut commands.entity(entity), assets);
    }

    pub fn yield_amount(&self) -> u32 {
        if let FarmState::NotPlanted = self.state {
            return 0;
        }

        let basic_yield = config().farming.basic_yield_percent * config().farming.max_yield;
        let rest_yield = config().farming.max_yield - basic_yield;

        let max_tendings = config().farming.growth_days; // 1 tending per day
        let tendings_percent = if self.tendings_done == 0 {
            0.0
        } else {
            (self.tendings_done as f32).min(max_tendings) / max_tendings
        };

        (basic_yield + (rest_yield * tendings_percent)).round() as u32
    }

    pub fn new_tending_rest_timer() -> Timer {
        Timer::from_seconds(
            hours_to_seconds(config().farming.tending_rest_hours),
            TimerMode::Once,
        )
    }
}

fn workable_props(farm_state: &FarmState) -> (Option<WorkKind>, f32) {
    match farm_state {
        FarmState::NotPlanted => (
            Some(WorkKind::FarmPlanting),
            config().farming.planting_hours,
        ),
        FarmState::Planted(_) => (Some(WorkKind::FarmTending), config().farming.tending_hours),
        FarmState::Grown => (
            Some(WorkKind::FarmHarvest),
            config().farming.harvesting_hours,
        ),
        FarmState::Harvested(_) => (None, 0.0),
    }
}

#[derive(Message, Debug)]
pub struct FarmProgressMessage(pub Entity);

#[derive(Message, Debug)]
pub struct FarmTendedMessage(pub Entity);
