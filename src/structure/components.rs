use super::*;

#[derive(Component)]
pub struct Warehouse {}

#[derive(Component, Default)]
pub struct FarmTile {
    pub state: FarmTileState,
}

impl FarmTile {
    pub fn progress_state(&mut self) {
        self.state = match &self.state {
            FarmTileState::NotPlanted => FarmTileState::Planted,
            FarmTileState::Planted => FarmTileState::Grown,
            FarmTileState::Grown => FarmTileState::Harvested,
            FarmTileState::Harvested => FarmTileState::NotPlanted,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub enum FarmTileState {
    #[default]
    NotPlanted,
    Planted,
    Grown,
    Harvested,
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
                sprite_bundle,
                Workable::new(hours_to_seconds(CONFIG.work_amount.farm_tile)),
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
            FarmTileState::Planted => assets.wheat.clone(),
            _ => assets.dirt.clone(),
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

#[derive(Component)]
pub struct House {}

// #[derive(Bundle)]
// pub struct StructureBundle {
//     pub structure: Structure,
//     pub name: Name,
//     // pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
//     pub sprite_bundle: SpriteBundle,
// }
