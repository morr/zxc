use bevy::ecs::system::EntityCommands;

use super::*;

#[derive(Component, Default)]
pub struct TileUIMarker {}

#[derive(Component, Default)]
pub struct TileComponentUIMarker {}

#[derive(Component, Default)]
pub struct TileTextUIMarker {}

pub fn render_tile_ui(
    hover_container_ui_commands: &mut EntityCommands,
    grid_tile: IVec2,
    font_assets: &Res<FontAssets>,
) {
    hover_container_ui_commands.with_children(|parent| {
        parent
            .spawn(render_entity_node_bunlde::<TileUIMarker>())
            .with_children(|parent| {
                parent
                    .spawn(render_entity_component_node_bunlde::<TileComponentUIMarker>())
                    .with_children(|parent| {
                        parent.spawn(headline_text_bundle("Tile", font_assets));
                        parent.spawn(property_text_bundle::<TileTextUIMarker>(
                            format!("{:?}", grid_tile),
                            font_assets,
                        ));
                    });
            });
    });
}
