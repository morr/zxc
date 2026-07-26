use crate::map::grid_tile_edge_to_world;
use crate::navigation::Navmesh;
use crate::navigation::Navtile;
use crate::navigation::grid_tile_to_navmesh_index;
use crate::prelude::*;
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

#[derive(Debug, Clone, Copy, Reflect)]
pub struct Tile {
    pub grid_tile: IVec2,
    pub kind: TileKind,
    pub height_noise: f32,
    pub humidity_noise: f32,
    pub props_noise: f32,
}

/// All map tiles as plain data, indexed by grid coordinates. Tiles are not
/// entities: rendering is done by `TilemapChunk` chunks, and tile lookups
/// (hover UI, noise debug) read this resource.
#[derive(Resource)]
pub struct TileGrid {
    size: usize,
    tiles: Vec<Tile>,
}

impl TileGrid {
    pub fn from_grid(grid: &[Vec<Tile>]) -> Self {
        let size = config().grid.size as usize;
        let mut tiles = Vec::with_capacity(size * size);
        // placeholder to allow index-based writes
        tiles.resize(
            size * size,
            Tile {
                grid_tile: IVec2::ZERO,
                kind: TileKind::DeepWater,
                height_noise: 0.,
                humidity_noise: 0.,
                props_noise: 0.,
            },
        );

        let mut result = Self { size, tiles };
        for row in grid.iter() {
            for tile in row.iter() {
                let index = result.index_of(tile.grid_tile);
                result.tiles[index] = *tile;
            }
        }
        result
    }

    fn index_of(&self, grid_tile: IVec2) -> usize {
        grid_tile_to_navmesh_index(grid_tile.x) * self.size
            + grid_tile_to_navmesh_index(grid_tile.y)
    }

    pub fn get(&self, grid_tile: IVec2) -> Option<&Tile> {
        let x = grid_tile.x + config().grid.half_size;
        let y = grid_tile.y + config().grid.half_size;
        if x < 0 || y < 0 || x >= self.size as i32 || y >= self.size as i32 {
            return None;
        }
        self.tiles.get(x as usize * self.size + y as usize)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
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

    pub fn sprite_transform(&self, maybe_offset: Option<Vec2>) -> Transform {
        let offset = maybe_offset.unwrap_or_default(); // Vec2::ZERO
        Transform::from_xyz(
            grid_tile_edge_to_world(self.grid_tile.x)
                + grid_tile_edge_to_world(self.width) / 2.
                + offset.x * config().tile.size,
            grid_tile_edge_to_world(self.grid_tile.y)
                + grid_tile_edge_to_world(self.width) / self.aspect_ratio / 2.
                + offset.y * config().tile.size,
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
