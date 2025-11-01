use super::*;

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum TileKind {
    DeepWater,
    ShallowWater,
    Sand,
    Grass,
    Forest,
    Mountain,
    RockyDirt,
    Dirt,
    FertileDirt,
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct Tile {
    pub grid_tile: IVec2,
    pub kind: TileKind,
    pub height_noise: f32,
    pub humidity_noise: f32,
    pub props_noise: f32,
}

impl Tile {
    pub fn texture(&self, assets: &Res<TextureAssets>) -> Handle<Image> {
        match self.kind {
            TileKind::ShallowWater => assets.shallow_water.clone(),
            TileKind::DeepWater => assets.deep_water.clone(),
            TileKind::Sand => assets.sand.clone(),
            TileKind::Grass => assets.grass.clone(),
            TileKind::Forest => assets.forest.clone(),
            TileKind::Mountain => assets.mountain.clone(),
            TileKind::RockyDirt => assets.rocky_dirt.clone(),
            TileKind::Dirt => assets.dirt.clone(),
            TileKind::FertileDirt => assets.fertile_dirt.clone(),
        }
    }
}

#[derive(Event, Debug)]
pub struct RebuildMapEvent {
    pub generator_kind: GeneratorKind,
}

#[derive(Event, Debug)]
pub struct RebuildMapCompleteEvent;

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum GeneratorKind {
    PerlinNoise,
}

pub struct TileItem {
    pub grid_tile: IVec2,
    pub width: i32,
    pub height: i32,
    pub aspect_ratio: f32,
    pub z_index: f32,
    pub movement_cost: f32,
}

impl TileItem {
    pub fn sprite_size(&self) -> Vec2 {
        let world_width = grid_tile_edge_to_world(self.width);
        Vec2::new(world_width, world_width / self.aspect_ratio)
    }

    pub fn sprite_transform(&self) -> Transform {
        Transform::from_xyz(
            grid_tile_edge_to_world(self.grid_tile.x) + grid_tile_edge_to_world(self.width) / 2.,
            grid_tile_edge_to_world(self.grid_tile.y) + grid_tile_edge_to_world(self.width) / self.aspect_ratio / 2.,
            self.z_index,
        )
    }

    pub fn sync_navmesh<T: 'static>(&self, id: Entity, navmesh: &mut Navmesh) {
        navmesh.update_cost(
            (self.grid_tile.x)..(self.grid_tile.x + self.width),
            (self.grid_tile.y)..(self.grid_tile.y + self.height),
            Navtile::config_cost_to_pathfinding_cost(self.movement_cost),
        );
        navmesh.add_occupant::<T>(&id, self.grid_tile.x, self.grid_tile.y);
    }
}
