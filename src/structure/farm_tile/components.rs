use bevy::ecs::system::EntityCommands;

use super::*;

macro_rules! farm_tile_states {
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
        pub enum FarmTileState {
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


        pub mod farm_tile_state {
            use bevy::{prelude::*};

            $(
                #[derive(Component)]
                pub struct $name;
            )*
        }

        impl FarmTile {
            pub fn change_state(
                &mut self,
                new_state: FarmTileState,
                entity: Entity,
                commands: &mut Commands,
                state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<FarmTileState>>,
            ) {
                // println!("FarmTileState {:?}=>{:?}", self.state, new_state);

                match &self.state {
                    $(FarmTileState::$name $( ($match_field) )? => {
                        commands.entity(entity).remove::<farm_tile_state::$name>();
                    },)*
                }

                self.state = new_state;

                match &self.state {
                    $(FarmTileState::$name $( ($match_field) )? => {
                        commands.entity(entity).insert(farm_tile_state::$name);
                    },)*
                }

                state_change_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
            }
        }
    };
}

farm_tile_states!(
    (NotPlanted),
    (
        Planted,
        struct PlantedState {
            growth_timer: Timer,
            tendings_done: u32,
            next_tending_timer: Option<Timer>
        },
        _a
    ),
    (Grown),
    (Harvested),
);

#[derive(Debug, Component, Reflect)]
pub struct FarmTile {
    pub state: FarmTileState,
}

impl Default for FarmTile {
    fn default() -> Self {
        Self {
            state: FarmTileState::NotPlanted,
        }
    }
}

impl FarmTile {
    pub fn progress_state(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
        transform: &Transform,
        assets: &Res<FarmAssets>,
        state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<FarmTileState>>,
    ) {
        let new_state = match &self.state {
            FarmTileState::NotPlanted => FarmTileState::Planted(PlantedState {
                growth_timer: Timer::from_seconds(
                    hours_to_seconds(CONFIG.farming.growth_time),
                    TimerMode::Once,
                ),
                tendings_done: 0,
                next_tending_timer: None,
            }),
            FarmTileState::Planted(_) => FarmTileState::Grown,
            FarmTileState::Grown => FarmTileState::Harvested,
            FarmTileState::Harvested => FarmTileState::NotPlanted,
        };
        // println!("progress_state {:?} => {:?}", self.state, new_state);
        self.change_state(new_state, entity, commands, state_change_event_writer);

        Self::sync_sprite_bundle(
            transform.world_pos_to_grid(),
            &self.state,
            &mut commands.entity(entity),
            assets,
        );
    }
}

#[derive(Event, Debug)]
pub struct FarmTileProgressEvent(pub Entity);

#[derive(Event, Debug)]
pub struct FarmTileTendingEvent(pub Entity);

impl FarmTile {
    pub fn spawn(
        commands: &mut Commands,
        assets: &Res<FarmAssets>,
        arc_navmesh: &mut Navmesh,
        grid_tile: IVec2,
        state_change_event_writer: &mut EventWriter<EntityStateChangeEvent<FarmTileState>>,
    ) {
        let farm_tile = Self::default();
        let state = farm_tile.state.clone();
        // let sprite_bundle = Self::sprite_bundle(&farm_tile.state, assets, grid_tile);
        let maybe_workable = Self::workable(&farm_tile.state);

        let mut entity_commands = commands.spawn((
            farm_tile,
            farm_tile_state::NotPlanted,
            Name::new("FarmTile"),
        ));
        FarmTile::sync_sprite_bundle(grid_tile, &state, &mut entity_commands, assets);
        FarmTile::sync_workable(&state, &mut entity_commands);

        if let Some(workable) = maybe_workable {
            entity_commands.insert(workable);
        }

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
        state: &FarmTileState,
        entity_commands: &mut EntityCommands,
        assets: &Res<FarmAssets>,
    ) {
        let size = IVec2::new(FARM_TILE_SIZE, FARM_TILE_SIZE);
        let texture = match state {
            FarmTileState::NotPlanted => assets.not_planted.clone(),
            FarmTileState::Planted(_) => assets.planted.clone(),
            FarmTileState::Grown => assets.grown.clone(),
            FarmTileState::Harvested => assets.harvested.clone(),
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

    pub fn sync_workable(state: &FarmTileState, entity_commands: &mut EntityCommands) {
        let work_amount = match state {
            FarmTileState::NotPlanted => CONFIG.farming.planting_work_amount,
            FarmTileState::Planted(_) => CONFIG.farming.tending_work_amount,
            FarmTileState::Grown => CONFIG.farming.harvesting_work_amount,
            FarmTileState::Harvested => CONFIG.farming.cleaning_work_amount,
        };

        entity_commands.insert(Workable::new(hours_to_seconds(work_amount)));
    }
}
