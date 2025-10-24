use bevy::ecs::system::EntityCommands;

use super::*;

#[derive(Component)]
pub struct TileUIMarker {
    #[allow(dead_code)]
    tile_id: Entity,
}

impl TargetEntityUiMarker for TileUIMarker {
    fn new(tile_id: Entity) -> Self {
        Self { tile_id }
    }
}

#[derive(Component, Default)]
pub struct TileComponentUIMarker {}

#[derive(Component, Default)]
pub struct TileTextUIMarker {}

pub fn render_tile_ui(
    tile_id: Entity,
    container_ui_commands: &mut EntityCommands,
    tile: &Tile,
    grid_tile: IVec2,
    font_assets: &Res<FontAssets>,
    opacity: UiOpacity,
) {
    container_ui_commands.with_children(|parent| {
        parent.spawn((
            render_entity_node_bunlde::<TileUIMarker>(tile_id, opacity),
            children![(
                render_entity_component_node_bunlde::<TileComponentUIMarker>(),
                children![
                    headline_text_bundle(format!("Tile {:?}", tile_id), font_assets,),
                    property_text_bundle::<TileTextUIMarker>(
                        format!("{:?}", grid_tile),
                        font_assets,
                    ),
                    property_text_bundle::<TileTextUIMarker>(
                        format!("{:?}", tile.kind),
                        font_assets,
                    ),
                    property_text_bundle::<TileTextUIMarker>(
                        format!("noise: {:.4}", tile.height_noise),
                        font_assets,
                    ),
                ],
            ),],
        ));
    });
}
