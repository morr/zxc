use bevy::prelude::*;

pub mod components;
use components::*;

use crate::{utils::tile_pos_to_world, GRID_COLS, GRID_ROWS, TILE_SIZE, TILE_Z_INDEX};

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // let mut rng = rand::thread_rng();
        // app.insert_resource(GroundTiles(HashSet::new()))
        //     .insert_resource(CurrentChunks(HashMap::new()))
        //     .insert_resource(GenerationSeed(rng.gen()))
        //     .add_systems(Update, handle_terrain_reset_event)
        //     .add_systems(Update, despawn_chunks)
        //     .add_systems(
        //         Update,
        //         clean_ground_tiles.run_if(on_timer(Duration::from_secs_f32(2.0))),
        //     )
        //     .add_systems(Update, handle_player_chunk_update_event)
        //     .add_event::<ResetTerrainEvent>();
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // let mesh = Mesh::from(Rectangle::new(TILE_SIZE, TILE_SIZE));
    // let material = ColorMaterial::from(Color::rgb(0.5, 0.5, 0.5));

    // let mesh_handle = meshes.add(mesh);
    // let material_handle = materials.add(material);

    // let texture_handle = asset_server.load("sprites/grass/Grass_08-128x128.png");
    // let texture_handle = asset_server.load("sprites/grass/Grass_21-128x128.png");
    // let texture_handle = asset_server.load("sprites/grass/Grass_21-128x128.png");
    // let texture_handle = asset_server.load("sprites/grass/Grass_23-128x128.png");
    // let texture_handle = asset_server.load("sprites/grass/Grass_24-128x128.png");
    // https://screamingbrainstudios.itch.io/tiny-texture-pack/download/eyJpZCI6MTAzMzEyOSwiZXhwaXJlcyI6MTcxMDc5ODI3OX0%3d.%2f%2bodleBeo8EbYeM%2bKnn3UZPKq2U%3d
    let texture_handle = asset_server.load("sprites/grass.png");

    for x in 0..GRID_COLS {
        for y in 0..GRID_ROWS {
            // commands.spawn((
            //     MaterialMesh2dBundle {
            //         mesh: mesh_handle.clone().into(),
            //         material: material_handle.clone(),
            //         transform: Transform::from_xyz(
            //             tile_pos_to_world(x as f32),
            //             tile_pos_to_world(y as f32),
            //             TILE_Z_INDEX,
            //         ),
            //         ..default()
            //     },
            //     TileComponent,
            // ));
            commands.spawn((
                SpriteBundle {
                    texture: texture_handle.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                        ..default()
                    },
                    // mesh: mesh_handle.clone().into(),
                    // material: material_handle.clone(),
                    transform: Transform::from_xyz(
                        tile_pos_to_world(x as f32),
                        tile_pos_to_world(y as f32),
                        TILE_Z_INDEX,
                    ),
                    ..default()
                },
                TileComponent,
            ));
        }
    }
}
