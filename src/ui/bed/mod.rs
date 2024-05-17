use bevy::ecs::system::EntityCommands;

use super::*;

#[derive(Component)]
pub struct BedUIMarker {
    #[allow(dead_code)]
    bed_id: Entity
}

impl TargetEntityUiMarker for BedUIMarker {
    fn new(bed_id: Entity) -> Self {
        Self { bed_id }
    }

}

#[derive(Component, Default)]
pub struct BedComponentUIMarker {}

#[derive(Component, Default)]
pub struct BedTextUIMarker {}

pub fn render_bed_ui(
    bed_id: Entity,
    container_ui_commands: &mut EntityCommands,
    grid_bed: IVec2,
    font_assets: &Res<FontAssets>,
    opacity: UiOpacity,
) {
    container_ui_commands.with_children(|parent| {
        parent
            .spawn(render_entity_node_bunlde::<BedUIMarker>(bed_id, opacity))
            .with_children(|parent| {
                parent
                    .spawn(render_entity_component_node_bunlde::<BedComponentUIMarker>())
                    .with_children(|parent| {
                        parent.spawn(headline_text_bundle(format!("Bed {:?}", bed_id), font_assets));
                        parent.spawn(property_text_bundle::<BedTextUIMarker>(
                            format!("{:?}", grid_bed),
                            font_assets,
                        ));
                    });
            });
    });
}
