use bevy::ecs::system::EntityCommands;

use super::*;

#[derive(Component)]
pub struct BedUIMarker {
    #[allow(dead_code)]
    bed_id: Entity,
}

impl TargetEntityUiMarker for BedUIMarker {
    fn new(bed_id: Entity) -> Self {
        Self { bed_id }
    }
}

#[derive(Component, Default)]
pub struct BedComponentUIMarker {}

#[derive(Component, Default)]
pub struct BedOwnerUIMarker {}

pub struct UiBedPlugin;

impl Plugin for UiBedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_bed_ui.run_if(in_state(AppState::Playing)),
        );
    }
}

pub fn render_bed_ui(
    bed_id: Entity,
    container_ui_commands: &mut EntityCommands,
    bed: &Bed,
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
                        parent.spawn(headline_text_bundle(
                            format!("Bed {:?}", bed_id),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<BedOwnerUIMarker>(
                            bed_owner_text(bed),
                            font_assets,
                        ));
                    });
            });
    });
}

fn update_bed_ui(
    ui_query: Query<(Entity, &BedUIMarker)>,
    texts_query: Query<(Entity, Option<&BedOwnerUIMarker>), Or<(With<BedOwnerUIMarker>,)>>,
    mut writer: TextUiWriter,
    components_query: Query<&Bed>,
    children_query: Query<&Children>,
) {
    for (ui_id, ui_marker) in ui_query.iter() {
        if let Ok(bed) = components_query.get(ui_marker.bed_id) {
            if let Ok(children) = children_query.get(ui_id) {
                for &child in children.iter() {
                    update_text_markers_recursive(child, bed, &mut writer, &texts_query, &children_query);
                }
            }
        }
    }
}

fn update_text_markers_recursive(
    entity: Entity,
    bed: &Bed,
    writer: &mut TextUiWriter,
    texts_query: &Query<(Entity, Option<&BedOwnerUIMarker>), Or<(With<BedOwnerUIMarker>,)>>,
    children_query: &Query<&Children>,
) {
    if let Ok((bed_entity, bed_owner_marker)) = texts_query.get(entity) {
        if bed_owner_marker.is_some() {
            *writer.text(bed_entity, 0) = bed_owner_text(bed);
        }
    }

    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            update_text_markers_recursive(child, bed, writer, texts_query, children_query);
        }
    }
}

fn bed_owner_text(bed: &Bed) -> String {
    format!("owner: {:?}", bed.owner)
}
