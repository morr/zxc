use bevy::ecs::system::EntityCommands;

use super::*;

#[derive(Component, Default)]
pub struct TileUIMarker {}

#[derive(Component, Default)]
pub struct TileComponentUIMarker {}

#[derive(Component, Default)]
pub struct TileTextUIMarker {}

pub fn render_tile_ui(
    container_ui_commands: &mut EntityCommands,
    tile: &Tile,
    grid_tile: IVec2,
    font_assets: &Res<FontAssets>,
    opacity: UiOpacity,
) {
    container_ui_commands.with_children(|parent| {
        parent.spawn((
            render_node_bunlde::<TileUIMarker>(opacity),
            children![(
                render_entity_component_node_bunlde::<TileComponentUIMarker>(),
                children![
                    headline_text_bundle(format!("Tile {:?}", grid_tile), font_assets,),
                    property_text_bundle::<TileTextUIMarker>(
                        format!("{:?}", grid_tile),
                        font_assets,
                    ),
                    property_text_bundle::<TileTextUIMarker>(
                        format!("{:?}", tile.kind),
                        font_assets,
                    ),
                    property_text_bundle::<TileTextUIMarker>(
                        format!("height noise: {:.4}", tile.height_noise),
                        font_assets,
                    ),
                    property_text_bundle::<TileTextUIMarker>(
                        format!("humidity noise: {:.4}", tile.humidity_noise),
                        font_assets,
                    ),
                    property_text_bundle::<TileTextUIMarker>(
                        format!("props noise: {:.4}", tile.props_noise),
                        font_assets,
                    ),
                ],
            ),],
        ));
    });
}
