use bevy::sprite_render::{AlphaMode2d, TileData, TilemapChunk, TilemapChunkTileData};
use rand::Rng;

use super::*;

/// The map is split into square render chunks of this many tiles per side.
/// Must divide `config().grid.size` evenly.
pub const MAP_CHUNK_SIZE: usize = 25;

/// Marker with the chunk position in chunk coordinates, used to rebuild
/// `TilemapChunkTileData` from `TileGrid` in place.
#[derive(Component)]
pub struct MapChunk {
    pub chunk_x: usize,
    pub chunk_y: usize,
}

pub fn generate_map(
    mut commands: Commands,
    tree_assets: Res<TreeAssets>,
    map_config: Res<MapGeneratorConfig>,
    tileset: Res<TilesetHandle>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    let mut navmesh = arc_navmesh.write();
    let grid = generator::perlin_noise::generate(&map_config);
    let tile_grid = TileGrid::from_grid(&grid);

    spawn_chunks(&mut commands, &tileset, &tile_grid);
    spawn_trees(&mut commands, &tree_assets, &mut navmesh, &grid);

    commands.insert_resource(tile_grid);
}

fn spawn_chunks(commands: &mut Commands, tileset: &TilesetHandle, tile_grid: &TileGrid) {
    let grid_size = config().grid.size as usize;
    assert_eq!(
        grid_size % MAP_CHUNK_SIZE,
        0,
        "grid.size must be divisible by MAP_CHUNK_SIZE"
    );
    let chunks_per_side = grid_size / MAP_CHUNK_SIZE;
    let tile_size = config().tile.size;
    let chunk_world_size = MAP_CHUNK_SIZE as f32 * tile_size;

    for chunk_x in 0..chunks_per_side {
        for chunk_y in 0..chunks_per_side {
            // chunk mesh is centered on its transform
            let left =
                grid_tile_edge_to_world(navmesh_index_to_grid_tile(chunk_x * MAP_CHUNK_SIZE));
            let bottom =
                grid_tile_edge_to_world(navmesh_index_to_grid_tile(chunk_y * MAP_CHUNK_SIZE));

            commands.spawn((
                MapChunk { chunk_x, chunk_y },
                TilemapChunk {
                    chunk_size: UVec2::splat(MAP_CHUNK_SIZE as u32),
                    tile_display_size: UVec2::splat(tile_size as u32),
                    tileset: tileset.0.clone(),
                    alpha_mode: AlphaMode2d::Opaque,
                },
                chunk_tile_data(tile_grid, chunk_x, chunk_y),
                Transform::from_xyz(
                    left + chunk_world_size / 2.,
                    bottom + chunk_world_size / 2.,
                    TILE_Z_INDEX,
                ),
            ));
        }
    }
}

/// Builds chunk data from the tile grid. Data is row-major starting from the
/// BOTTOM-LEFT tile of the chunk (verified empirically: the chunk origin is
/// bottom-left since bevy 0.18).
fn chunk_tile_data(tile_grid: &TileGrid, chunk_x: usize, chunk_y: usize) -> TilemapChunkTileData {
    let mut data = Vec::with_capacity(MAP_CHUNK_SIZE * MAP_CHUNK_SIZE);

    for row in 0..MAP_CHUNK_SIZE {
        for col in 0..MAP_CHUNK_SIZE {
            let grid_tile = IVec2::new(
                navmesh_index_to_grid_tile(chunk_x * MAP_CHUNK_SIZE + col),
                navmesh_index_to_grid_tile(chunk_y * MAP_CHUNK_SIZE + row),
            );
            let tile = tile_grid
                .get(grid_tile)
                .expect("chunk tile out of tile grid range");
            data.push(Some(TileData::from_tileset_index(
                tile.kind.tileset_index(),
            )));
        }
    }

    TilemapChunkTileData(data)
}

fn spawn_trees(
    commands: &mut Commands,
    assets: &Res<TreeAssets>,
    _navmesh: &mut Navmesh,
    grid: &[Vec<Tile>],
) {
    let mut rng = rand::rng();

    let tree_variants: [(Handle<Image>, f32); _] = [
        (assets.tree_1x2_1.clone(), TreeAssets::ASPECT_RATIO_1X2),
        (assets.tree_1x2_2.clone(), TreeAssets::ASPECT_RATIO_1X2),
        (assets.tree_1x2_3.clone(), TreeAssets::ASPECT_RATIO_1X2),
        (assets.tree_1x2_4.clone(), TreeAssets::ASPECT_RATIO_1X2),
        (assets.tree_1x2_5.clone(), TreeAssets::ASPECT_RATIO_1X2),
        (assets.tree_1x2_6.clone(), TreeAssets::ASPECT_RATIO_1X2),
        (assets.tree_1x2_7.clone(), TreeAssets::ASPECT_RATIO_1X2),
        (assets.tree_1x2_8.clone(), TreeAssets::ASPECT_RATIO_1X2),
        (assets.tree_1x2_9.clone(), TreeAssets::ASPECT_RATIO_1X2),
        (assets.tree_1x2_10.clone(), TreeAssets::ASPECT_RATIO_1X2),
        (assets.tree_1x3_1.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_2.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_3.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_4.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_5.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_6.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_7.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_1x3_8.clone(), TreeAssets::ASPECT_RATIO_1X3),
        (assets.tree_2x3_1.clone(), TreeAssets::ASPECT_RATIO_2X3),
        (assets.tree_2x3_2.clone(), TreeAssets::ASPECT_RATIO_2X3),
        (assets.tree_2x3_3.clone(), TreeAssets::ASPECT_RATIO_2X3),
        (assets.tree_2x3_4.clone(), TreeAssets::ASPECT_RATIO_2X3),
        (assets.tree_3x4_1.clone(), TreeAssets::ASPECT_RATIO_3X4),
        (assets.tree_3x4_2.clone(), TreeAssets::ASPECT_RATIO_3X4),
    ];

    for row in grid.iter().rev() {
        for tile in row.iter().rev() {
            // if tile.height_noise >= 0.6 && tile.humidity_noise >= 0.6 && tile.props_noise >= 0.6 {
            if tile.height_noise >= 0.553 && tile.props_noise >= 0.479 {
                let (tree_image, aspect_ratio) = tree_variants[rng.random_range(0..14)].clone();
                let tile_item = TileItem {
                    grid_tile: tile.grid_tile,
                    width: 1,
                    height: 1,
                    aspect_ratio,
                    z_index: PROP_Z_INDEX,
                    movement_cost: 1.0,
                };
                let random_angle: f32 = rng.random_range(0.0..360.0);

                commands.spawn((
                    Sprite {
                        image: tree_image,
                        custom_size: Some(tile_item.sprite_size()),
                        ..default()
                    },
                    tile_item.sprite_transform(Some(Vec2 {
                        x: random_angle.cos() * 0.35,
                        y: random_angle.sin().abs() * 0.5,
                    })),
                ));
            }
        }
    }
}

pub fn on_rebuild_map(
    event: On<RebuildMapEvent>,
    mut commands: Commands,
    map_config: Res<MapGeneratorConfig>,
    mut tile_grid: ResMut<TileGrid>,
    mut chunks_query: Query<(&MapChunk, &mut TilemapChunkTileData)>,
) {
    let RebuildMapEvent { generator_kind } = *event;

    let grid = match generator_kind {
        GeneratorKind::PerlinNoise => generator::perlin_noise::generate(&map_config),
    };
    *tile_grid = TileGrid::from_grid(&grid);

    // chunk entities live forever; rebuilding only mutates their tile data
    for (map_chunk, mut tile_data) in chunks_query.iter_mut() {
        *tile_data = chunk_tile_data(&tile_grid, map_chunk.chunk_x, map_chunk.chunk_y);
    }

    commands.trigger(log_event!(RebuildMapCompleteEvent));
}
