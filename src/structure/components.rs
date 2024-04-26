use super::*;

#[derive(Component)]
pub struct Warehouse {}

#[derive(Component)]
pub struct FarmTile {}

impl FarmTile {
    pub fn spawn(
        commands: &mut Commands,
        assets: &Res<TextureAssets>,
        arc_navmesh: &mut Navmesh,
        work_queue: &mut ResMut<WorkQueue>,
        grid_tile: IVec2,
        size: IVec2,
    ) {
        commands.spawn((
            FarmTile {},
            Workable::default(),
            Name::new("FarmTile"),
            SpriteBundle {
                texture: assets.dirt.clone(),
                sprite: Sprite {
                    custom_size: Some(size.grid_tile_edge_to_world()),
                    ..default()
                },
                transform: Transform::from_translation(
                    (grid_tile.grid_tile_edge_to_world() + size.grid_tile_edge_to_world() / 2.0)
                        .extend(STRUCTURE_Z_INDEX),
                ),
                ..default()
            },
        ));

        arc_navmesh.update_cost(
            grid_tile.x..grid_tile.x + size.x,
            grid_tile.y..grid_tile.y + size.y,
            Some((3.0 * COST_MULTIPLIER) as i32),
        );

        // Adding the task for the farm tile to the work queue
        let task = Task {
            kind: TaskKind::Farming,
            tile: grid_tile,
        };
        work_queue.add_task(task);
    }
}

#[derive(Component)]
pub struct House {}

// #[derive(Bundle)]
// pub struct StructureBundle {
//     pub structure: Structure,
//     pub name: Name,
//     // pub mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
//     pub sprite_bundle: SpriteBundle,
// }
