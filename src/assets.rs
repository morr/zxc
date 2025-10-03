use crate::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppState::Loading)
                .finally_init_resource::<AssetsCollection>()
                .finally_init_resource::<MeshesCollection>()
                .load_collection::<FontAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<IconAssets>()
                .load_collection::<FarmAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub fira: Handle<Font>,
}

// thttps://itch.io/game-assets/free/tag-textures
#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "sprites/tile/deep_water.png")]
    pub deep_water: Handle<Image>,
    // https://www.the3rdsequence.com/texturedb/texture/44/clear+sea+water/
    #[asset(path = "sprites/tile/shallow_water.png")]
    pub shallow_water: Handle<Image>,
    #[asset(path = "sprites/tile/sand.png")]
    pub sand: Handle<Image>,
    // https://screamingbrainstudios.itch.io/tiny-texture-pack/download/eyJpZCI6MTAzMzEyOSwiZXhwaXJlcyI6MTcxMDc5ODI3OX0%3d.%2f%2bodleBeo8EbYeM%2bKnn3UZPKq2U%3d
    #[asset(path = "sprites/tile/grass.png")]
    pub grass: Handle<Image>,
    #[asset(path = "sprites/tile/forest.png")]
    pub forest: Handle<Image>,
    #[asset(path = "sprites/tile/mountain.png")]
    pub mountain: Handle<Image>,
    #[asset(path = "sprites/tile/rocky_dirt.png")]
    pub rocky_dirt: Handle<Image>,
    #[asset(path = "sprites/tile/dirt.png")]
    pub dirt: Handle<Image>,
    #[asset(path = "sprites/tile/fertile_dirt.png")]
    pub fertile_dirt: Handle<Image>,

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

    // prompt: round basin full of deep water, rough stone and stone border, minimal details, top down view, pixel art, texture for sprite, clean water, transparent background
    //
    // negativeprompt: blurry, shadows,  hole at center, shadow, basin shadow
    // mechanisms, stairs, grass
    // water pouring, water waves. water ripples, water reflections
    // pebbles inside water
    // water outside basin
    //
    // disfigured, seed: 1016497133, steps: 100, cfgscale: 7, aspectratio: 1:1, width: 1024, height: 1024, sampler: dpmpp_sde_gpu, scheduler: karras, model: pixelArtDiffusionXL_spriteShaper.safetensors, swarm_version: 0.6.2.0, date: 2024-04-25, generation_time: 0.02 (prep) and 54.20 (gen) seconds,
    #[asset(path = "sprites/well.png")]
    pub well: Handle<Image>,

    // prompt: bed on white background, (primitive bed for single person:1.2), hay pillow, skin blanket, (white background:1.1), top down view, (pixel art:1.1), for texture, low details
    //
    // negativeprompt: blurry, shadows, disfigured, two pillows, extra details, grass, tree, extra furniture, carpet, clothes, cloth pillow
    //
    // seed: 1406256067, steps: 60, cfgscale: 7, aspectratio: 1:1, width: 1024, height: 1024, sampler: euler, scheduler: karras, model: pixelArtDiffusionXL_spriteShaper.safetensors, swarm_version: 0.6.2.0, date: 2024-05-17, generation_time: 0.02 (prep) and 16.70 (gen) seconds,
    #[asset(path = "sprites/bed.png")]
    pub bed: Handle<Image>,

    // prompt: primitive empty storage place made of wood, top down view, pixel art, white background, low details
    //
    // negativeprompt: blurry, shadows, disfigured, extra details, grass, tree, extra furniture, extra details
    //
    // images: 25, seed: 2128735519, steps: 20, cfgscale: 7, aspectratio: 1:1, width: 1024, height: 1024, sampler: ddim, scheduler: karras, model: pixelArtDiffusionXL_spriteShaper.safetensors, swarm_version: 0.6.2.0, date: 2024-06-09, generation_time: 12.61 (prep) and 5.67 (gen) seconds,
    #[asset(path = "sprites/storage.png")]
    pub storage: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct IconAssets {
    // https://ghostpixxells.itch.io/pixelfood
    #[asset(path = "icons/bread.png")]
    pub bread: Handle<Image>,

    // https://www.vecteezy.com/vector-art/6470690-team-pixel-art-business-icon
    // https://www.pinterest.com/pin/892416482393656144/
    #[asset(path = "icons/pawns.png")]
    pub pawns: Handle<Image>,
}

// https://itch.io/game-assets/free/tag-textures
#[derive(AssetCollection, Resource)]
pub struct FarmAssets {
    #[asset(path = "sprites/farm/not_planted.png")] // SBS - Tiny Texture Pack 2 - 128x128
    pub not_planted: Handle<Image>,

    // https://lynocs.itch.io/texture-pack
    // Grass/9/9_diffuseOriginal.bmp
    #[asset(path = "sprites/farm/planted.png")]
    pub planted: Handle<Image>,

    // prompt: ripe wheat, minimal details, top down view, pixel art, texture for sprite
    // negativeprompt: blurry, shadows,  disfigured
    // seed: 918053270, steps: 20, cfgscale: 7, aspectratio: 1:1, width: 1024, height: 1024, sampler: dpmpp_sde_gpu, scheduler: karras, model: pixelArtDiffusionXL_spriteShaper.safetensors, swarm_version: 0.6.2.0, date: 2024-05-02, generation_time: 2.05 (prep) and 12.14 (gen) seconds,
    #[asset(path = "sprites/farm/grown.png")]
    pub grown: Handle<Image>,

    #[asset(path = "sprites/farm/harvested.png")]
    pub harvested: Handle<Image>,
}

#[derive(Resource)]
pub struct AssetsCollection {
    pub pawn_idle: Handle<ColorMaterial>,
    pub pawn_moving: Handle<ColorMaterial>,
    pub pawn_pathfinding: Handle<ColorMaterial>,
    pub pawn_pathfinding_error: Handle<ColorMaterial>,
    pub pawn_working: Handle<ColorMaterial>,
    pub pawn_dead: Handle<ColorMaterial>,
    pub navmesh_passable: Handle<ColorMaterial>,
    pub navmesh_impassable: Handle<ColorMaterial>,
    pub food: Handle<ColorMaterial>,
}

impl FromWorld for AssetsCollection {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();

        Self {
            pawn_idle: materials.add(ColorMaterial::from(Color::from(
                Srgba::hex("f6f8fa").unwrap(),
            ))),
            pawn_moving: materials.add(ColorMaterial::from(Color::from(
                Srgba::hex("1193cf").unwrap(),
            ))),
            // pawn_moving: materials.add(ColorMaterial::from(Color::hex("e178c5").unwrap())),
            pawn_pathfinding: materials.add(ColorMaterial::from(Color::from(
                Srgba::hex("fb8f44").unwrap(),
            ))),
            pawn_pathfinding_error: materials.add(ColorMaterial::from(Color::from(
                Srgba::hex("ff0000").unwrap(),
            ))),
            pawn_working: materials.add(ColorMaterial::from(Color::from(
                Srgba::hex("74d61f").unwrap(),
            ))),
            pawn_dead: materials.add(ColorMaterial::from(Color::from(
                Srgba::hex("181a1c").unwrap(),
            ))),
            navmesh_passable: materials.add(ColorMaterial::from(Color::srgba(0.0, 0.0, 0.75, 0.5))),
            navmesh_impassable: materials
                .add(ColorMaterial::from(Color::srgba(1.0, 0.0, 0.0, 0.75))),
            food: materials.add(ColorMaterial::from(Color::from(
                Srgba::hex("fe9516").unwrap(),
            ))),
        }
    }
}

#[derive(Resource)]
pub struct MeshesCollection {
    pub pawn: Handle<Mesh>,
    pub food: Handle<Mesh>,
}

impl FromWorld for MeshesCollection {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();

        Self {
            pawn: meshes.add(Mesh::from(Circle::new(config().tile.size / 2.0 * 0.75))),
            food: meshes.add(Mesh::from(Rectangle::new(
                config().tile.size / 4.0,
                config().tile.size / 4.0,
            ))),
        }
    }
}
