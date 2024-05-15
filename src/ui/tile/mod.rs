use bevy::ecs::system::EntityCommands;

use super::*;

#[derive(Component)]
pub struct TileUIMarker {
    #[allow(dead_code)]
    tile_id: Entity
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
    grid_tile: IVec2,
    font_assets: &Res<FontAssets>,
) {
    container_ui_commands.with_children(|parent| {
        parent
            .spawn(render_entity_node_bunlde::<TileUIMarker>(tile_id))
            .with_children(|parent| {
                parent
                    .spawn(render_entity_component_node_bunlde::<TileComponentUIMarker>())
                    .with_children(|parent| {
                        parent.spawn(headline_text_bundle(format!("Tile {:?}", tile_id), font_assets));
                        parent.spawn(property_text_bundle::<TileTextUIMarker>(
                            format!("{:?}", grid_tile),
                            font_assets,
                        ));
                    });
            });
    });
}
