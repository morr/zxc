use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub mod components;
use components::*;

use crate::{GRID_COLS, GRID_ROWS, TILE_SIZE, TILE_Z_INDEX};

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh::from(Rectangle::new(TILE_SIZE, TILE_SIZE));
    let material = ColorMaterial::from(Color::rgb(0.5, 0.5, 0.5));

    let mesh_handle = meshes.add(mesh);
    let material_handle = materials.add(material);

    for x in 0..GRID_COLS {
        for y in 0..GRID_ROWS {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    material: material_handle.clone(),
                    transform: Transform::from_xyz(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, TILE_Z_INDEX),
                    ..default()
                },
                TileComponent,
            ));
        }
    }
}
