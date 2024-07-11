use super::*;

#[derive(Component, Debug, Clone)]
pub struct Tile {
    pub grid_tile: IVec2
}

impl Tile {
    pub fn texture(&self, assets: &Res<TextureAssets>) -> Handle<Image> {
        assets.grass.clone()
    }
}
