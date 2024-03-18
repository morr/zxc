use bevy::prelude::*;

pub mod components;
use components::*;

use crate::{GRID_COLS, GRID_ROWS};

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

fn spawn_map(mut commands: Commands) {
    for x in 0..GRID_COLS {
        for y in 0..GRID_ROWS {
            commands.spawn(TileComponent);
        }
    }
}
