use super::*;

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum TileKind {
    DeepWater,
    ShallowWater,
    Sand,
    Grass,
    Forest,
    Mountain,
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct Tile {
    pub grid_tile: IVec2,
    pub kind: TileKind,
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
        }
    }
}

#[derive(Event, Debug)]
pub struct RebuildMapEvent {
    pub generator_kind: GeneratorKind,
}

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum GeneratorKind {
    CellularAutomata,
    // MarkovJuniour
}
