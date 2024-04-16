use crate::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(WorldState::Loading)
                .init_resource::<AssetsCollection>()
                .load_collection::<TextureAssets>(),
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
pub struct TextureAssets {
    #[asset(path = "sprites/castle_complete.png")] // https://fin-nio.itch.io/pixel-houses
    pub castle: Handle<Image>,
    #[asset(path = "sprites/grass.png")] // https://itch.io/game-assets/free/tag-textures https://screamingbrainstudios.itch.io/tiny-texture-pack/download/eyJpZCI6MTAzMzEyOSwiZXhwaXJlcyI6MTcxMDc5ODI3OX0%3d.%2f%2bodleBeo8EbYeM%2bKnn3UZPKq2U%3d
    pub grass: Handle<Image>,
    #[asset(path = "sprites/dirt.png")] // SBS - Tiny Texture Pack 2 - 128x128
    pub dirt: Handle<Image>,
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
