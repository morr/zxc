use crate::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppState::Loading)
                .init_resource::<AssetsCollection>()
                .load_collection::<FontAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<FarmAssets>(),
        );
    }
}

#[derive(Resource)]
pub struct AssetsCollection {
    pub pawn_idle: Handle<ColorMaterial>,
    pub pawn_moving: Handle<ColorMaterial>,
    pub pawn_pathfinding: Handle<ColorMaterial>,
    pub pawn_pathfinding_error: Handle<ColorMaterial>,
    pub navmesh_passable: Handle<ColorMaterial>,
    pub navmesh_impassable: Handle<ColorMaterial>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub fira: Handle<Font>,
}

// https://itch.io/game-assets/free/tag-textures
#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    // https://screamingbrainstudios.itch.io/tiny-texture-pack/download/eyJpZCI6MTAzMzEyOSwiZXhwaXJlcyI6MTcxMDc5ODI3OX0%3d.%2f%2bodleBeo8EbYeM%2bKnn3UZPKq2U%3d
    #[asset(path = "sprites/grass.png")]
    pub grass: Handle<Image>,

    // https://fin-nio.itch.io/pixel-houses
    #[asset(path = "sprites/castle_complete.png")]
    pub castle: Handle<Image>,

    #[asset(path = "sprites/house_1.png")]
    pub house_1: Handle<Image>,

    #[asset(path = "sprites/house_2.png")]
    pub house_2: Handle<Image>,

    #[asset(path = "sprites/house_3.png")]
    pub house_3: Handle<Image>,

    #[asset(path = "sprites/house_4.png")]
    pub house_4: Handle<Image>,

    #[asset(path = "sprites/well.png")]
    pub well: Handle<Image>,
}

// https://itch.io/game-assets/free/tag-textures
#[derive(AssetCollection, Resource)]
pub struct FarmAssets {
    #[asset(path = "sprites/farm_tile/not_planted.png")] // SBS - Tiny Texture Pack 2 - 128x128
    pub not_planted: Handle<Image>,

    // https://lynocs.itch.io/texture-pack
    // Grass/9/9_diffuseOriginal.bmp
    #[asset(path = "sprites/farm_tile/planted.png")]
    pub planted: Handle<Image>,

    #[asset(path = "sprites/farm_tile/grown.png")]
    pub grown: Handle<Image>,

    #[asset(path = "sprites/farm_tile/harvested.png")]
    pub harvested: Handle<Image>,
}

impl FromWorld for AssetsCollection {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        Self {
            pawn_idle: materials.add(ColorMaterial::from(Color::hex("f6f8fa").unwrap())),
            pawn_moving: materials.add(ColorMaterial::from(Color::hex("1193cf").unwrap())),
            // pawn_moving: materials.add(ColorMaterial::from(Color::hex("e178c5").unwrap())),
            pawn_pathfinding: materials.add(ColorMaterial::from(Color::hex("fb8f44").unwrap())),
            pawn_pathfinding_error: materials.add(ColorMaterial::from(Color::RED)),
            navmesh_passable: materials.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.75, 0.5))),
            navmesh_impassable: materials
                .add(ColorMaterial::from(Color::rgba(1.0, 0.0, 0.0, 0.75))),
        }
    }
}
