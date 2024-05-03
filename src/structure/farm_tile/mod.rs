use super::*;

macro_rules! farm_tile_states {
    (
        $( ($name:ident $(, $turple_type:ty, $match_field:ident)?)),* $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq)]
        pub enum FarmTileState {
            $($name $(($turple_type))? ),*
        }

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
            }
        }
    };
}

farm_tile_states!((NotPlanted), (Planted, Timer, _a), (Grown), (Harvested),);

#[derive(Component)]
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
    ) {
        let new_state = match &self.state {
            FarmTileState::NotPlanted => FarmTileState::Planted(Timer::from_seconds(
                hours_to_seconds(CONFIG.work_amount.farm_tile_grow),
                TimerMode::Once,
            )),
            FarmTileState::Planted(_) => FarmTileState::Grown,
            FarmTileState::Grown => FarmTileState::Harvested,
            FarmTileState::Harvested => FarmTileState::NotPlanted,
        };
        // println!("progress_state {:?} => {:?}", self.state, new_state);
        self.change_state(new_state, entity, commands);

        let grid_tile = transform.translation.truncate().world_pos_to_grid();
        commands
            .entity(entity)
            .insert(FarmTile::sprite_bundle(&self.state, assets, grid_tile));

        // if new_state == FarmTileState::Grown {
        //     work_queue.add_task(Task {
        //         entity,
        //         kind: TaskKind::FarmTileHarvest,
        //         tile: grid_tile,
        //     });
        // }
    }
}

#[derive(Event, Debug)]
pub struct FarmTileProgressEvent(pub Entity);

impl FarmTile {
    pub fn spawn(
        commands: &mut Commands,
        assets: &Res<FarmAssets>,
        arc_navmesh: &mut Navmesh,
        work_queue: &mut ResMut<TasksQueue>,
        grid_tile: IVec2,
    ) {
        let farm_tile = Self::default();
        let sprite_bundle = Self::sprite_bundle(&farm_tile.state, assets, grid_tile);

        let entity = commands
            .spawn((
                farm_tile,
                farm_tile_state::NotPlanted,
                sprite_bundle,
                Workable::new(hours_to_seconds(CONFIG.work_amount.farm_tile_plant)),
                Name::new("FarmTile"),
            ))
            .id();

        arc_navmesh.update_cost(
            grid_tile.x..grid_tile.x + FARM_TILE_SIZE,
            grid_tile.y..grid_tile.y + FARM_TILE_SIZE,
            Some((3.0 * COST_MULTIPLIER) as i32),
        );

        work_queue.add_task(Task {
            entity,
            kind: TaskKind::FarmTilePlant,
            tile: grid_tile,
        });
    }

    pub fn sprite_bundle(
        state: &FarmTileState,
        assets: &Res<FarmAssets>,
        grid_tile: IVec2,
    ) -> SpriteBundle {
        let size = IVec2::new(FARM_TILE_SIZE, FARM_TILE_SIZE);
        let texture = match state {
            FarmTileState::NotPlanted => assets.not_planted.clone(),
            FarmTileState::Planted(_) => assets.planted.clone(),
            FarmTileState::Grown => assets.grown.clone(),
            FarmTileState::Harvested => assets.harvested.clone(),
        };

        SpriteBundle {
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
        }
    }
}

pub fn progress_farm_tile_state(
    mut query: Query<(&Transform, &mut FarmTile)>,
    mut commands: Commands,
    mut event_reader: EventReader<FarmTileProgressEvent>,
    assets: Res<FarmAssets>,
) {
    for event in event_reader.read() {
        let entity = event.0;
        let (transform, mut farm_tile) = query.get_mut(entity).unwrap();

        farm_tile.progress_state(entity, &mut commands, transform, &assets);
    }
}

pub fn progress_farm_tile_timer(
    time: Res<Time>,
    time_scale: Res<TimeScale>,
    mut query: Query<(Entity, &Transform, &mut FarmTile), With<farm_tile_state::Planted>>,
    mut commands: Commands,
    assets: Res<FarmAssets>,
) {
    for (entity, transform, mut farm_tile) in query.iter_mut() {
        let timer = match &mut farm_tile.state {
            FarmTileState::Planted(timer) => timer,
            _ => panic!("FarmTile must be in a timer-assigned state"),
        };
        timer.tick(time_scale.scale_to_duration(time.delta_seconds()));

        if timer.finished() {
            farm_tile.progress_state(entity, &mut commands, transform, &assets);
        }
    }
}
