use super::*;

// #[derive(Debug, Clone, Default, PartialEq)]
// pub enum FarmTileState {
//     #[default]
//     NotPlanted,
//     Planted,
//     Grown,
//     Harvested,
// }

macro_rules! farm_tile_states {
    // ($head:ident, $($tail:ident),*) => {
    ($($name:ident),*) => {
        #[derive(Component, Clone, Eq, PartialEq, Debug, Reflect)]
        pub enum FarmTileState {
            $(
                $name,
            )*
        }
        // #[derive(Debug, Clone, PartialEq, Default)]
        // pub enum FarmTileState {
        //     #[default]
        //     $head,
        //     $(
        //         $tail,
        //     )*
        // }
        //
        // $(
        //     #[derive(Component)]
        //     pub struct $name;
        // )*
    };
}

farm_tile_states!(NotPlanted, Planted, Grown, Harvested);

#[derive(Component)]
pub struct FarmTile {
    pub state: FarmTileState,
    pub grow_timer: Option<Timer>,
}

impl Default for FarmTile {
    fn default() -> Self {
        Self {
            state: FarmTileState::NotPlanted,
            grow_timer: None
        }
    }
}

impl FarmTile {
    pub fn progress_state(&mut self, _entity: Entity, _commands: &mut Commands) {
        self.state = match &self.state {
            FarmTileState::NotPlanted => FarmTileState::Planted,
            FarmTileState::Planted => FarmTileState::Grown,
            FarmTileState::Grown => FarmTileState::Harvested,
            FarmTileState::Harvested => FarmTileState::NotPlanted,
        };

        if self.state == FarmTileState::Planted {
            self.grow_timer = Some(Timer::from_seconds(
                hours_to_seconds(CONFIG.work_amount.farm_tile_grow),
                TimerMode::Once,
            ));
        }
    }
}

// macro_rules! define_farmtile_states {
//     (
//         ($first_enum_name:ident, $first_component_name:ident)
//         $( , $enum_name:ident, $component_name:ident )*
//     ) => {
//         #[derive(Debug, Clone, Default)]
//         pub enum FarmTileState {
//             #[default]
//             $first_enum_name,
//             $($enum_name $(($turple_type))? ),*
//         }
//
//         #[derive(Component)]
//         pub struct $first_component_name;
//
//         $(
//             #[derive(Component)]
//             pub struct $component_name;
//         )*
//
//        impl FarmTile {
//             pub fn change_state(
//                 &mut self,
//                 entity: Entity,
//                 // new_state: FarmTileState,
//                 commands: &mut Commands,
//                 // farmtile_state_event_writer: &mut EventWriter<EntityStateChangeEvent<FarmTileState>>,
//             ) {
//                 // println!("FarmTileState {:?}=>{:?}", self.state, new_state);
//                 // Remove the old state component
//                 match &self.state {
//                     FarmTileState::$first_enum_name => {
//                         commands.entity(entity).remove::<$first_component_name>();
//                     },
//                     $(FarmTileState::$enum_name $( ($match_field) )? => {
//                         commands.entity(entity).remove::<$component_name>();
//                     },)*
//                 }
//
//                 // Set the new state
//                 self.state = new_state;
//
//                 // Add the new component
//                 match &self.state {
//                     FarmTileState::$first_enum_name => {
//                         commands.entity(entity).insert($first_component_name);
//                     },
//                     $(FarmTileState::$enum_name $( ($match_field) )? => {
//                         commands.entity(entity).insert($component_name);
//                     },)*
//                 }
//
//                 // farmtile_state_event_writer.send(EntityStateChangeEvent(entity, self.state.clone()));
//             }
//         }
//     };
// }
//
// define_farm_tile_states!(
//     (NotPlanted, FarmTileNotPlanted)
//     (Planted, FarmTilePlanted)
//     (Grown, FarmTileGrown)
//     (Harvested, FarmTileHarvested)
// );

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

        // Adding the task for the farm tile to the work queue
        let task = Task {
            entity,
            kind: TaskKind::Farming,
            tile: grid_tile,
        };
        work_queue.add_task(task);
    }

    pub fn sprite_bundle(
        state: &FarmTileState,
        assets: &Res<FarmAssets>,
        grid_tile: IVec2,
    ) -> SpriteBundle {
        let size = IVec2::new(FARM_TILE_SIZE, FARM_TILE_SIZE);
        let texture = match state {
            FarmTileState::NotPlanted => assets.not_planted.clone(),
            FarmTileState::Planted => assets.planted.clone(),
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
    mut commands: Commands,
    mut query: Query<(&Transform, &mut FarmTile)>,
    mut event_reader: EventReader<FarmTileProgressEvent>,
    assets: Res<FarmAssets>,
) {
    for event in event_reader.read() {
        let farm_tile_entity = event.0;
        let (transform, mut farm_tile) = query.get_mut(farm_tile_entity).unwrap();
        let grid_tile = transform.translation.truncate().world_pos_to_grid();

        farm_tile.progress_state(farm_tile_entity, &mut commands);

        commands.entity(event.0).insert(FarmTile::sprite_bundle(
            &farm_tile.state,
            &assets,
            grid_tile,
        ));
    }
}

pub fn progress_farm_tile_timer(_commands: Commands, _query: Query<(&Transform, &mut FarmTile)>) {}
