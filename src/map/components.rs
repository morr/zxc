use super::*;

#[derive(Debug, Clone, Copy, Reflect, PartialEq, Eq)]
pub enum TileKind {
    Water,
    Grass,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Tile {
    pub grid_tile: IVec2,
    pub kind: TileKind,
}

impl Tile {
    pub fn texture(&self, assets: &Res<TextureAssets>) -> Handle<Image> {
        match self.kind {
            TileKind::Grass => assets.grass.clone(),
            TileKind::Water => assets.water.clone(),
        }
    }
}
