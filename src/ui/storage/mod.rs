use bevy::ecs::system::EntityCommands;

use super::*;

#[derive(Component)]
pub struct StorageUIMarker {
    #[allow(dead_code)]
    storage_id: Entity,
}

impl TargetEntityUiMarker for StorageUIMarker {
    fn new(storage_id: Entity) -> Self {
        Self { storage_id }
    }
}

#[derive(Component, Default)]
pub struct StorageComponentUIMarker {}

#[derive(Component, Default)]
pub struct StorageGridTileUIMarker {}

// #[derive(Component, Default)]
// pub struct StorageOwnerUIMarker {}

pub struct UiStoragePlugin;

impl Plugin for UiStoragePlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(
        //     FixedUpdate,
        //     update_storage_ui.run_if(in_state(AppState::Playing)),
        // );
    }
}

pub fn render_storage_ui(
    storage_id: Entity,
    container_ui_commands: &mut EntityCommands,
    _storage: &Storage,
    grid_tile: IVec2,
    font_assets: &Res<FontAssets>,
    opacity: UiOpacity,
) {
    container_ui_commands.with_children(|parent| {
        parent
            .spawn(render_entity_node_bunlde::<StorageUIMarker>(storage_id, opacity))
            .with_children(|parent| {
                parent
                    .spawn(render_entity_component_node_bunlde::<StorageComponentUIMarker>())
                    .with_children(|parent| {
                        parent.spawn(headline_text_bundle(
                            format!("Storage {:?}", storage_id),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<StorageGridTileUIMarker>(
                            format!("{:?}", grid_tile),
                            font_assets,
                        ));
                        // parent.spawn(property_text_bundle::<StorageOwnerUIMarker>(
                        //     storage_owner_text(storage),
                        //     font_assets,
                        // ));
                    });
            });
    });
}

// fn update_storage_ui(
//     ui_query: Query<(Entity, &StorageUIMarker)>,
//     mut texts: Query<(&mut Text, Option<&StorageOwnerUIMarker>), Or<(With<StorageOwnerUIMarker>,)>>,
//     components_query: Query<&Storage>,
//     children_query: Query<&Children>,
// ) {
//     for (ui_id, ui_marker) in ui_query.iter() {
//         if let Ok(storage) = components_query.get(ui_marker.storage_id) {
//             if let Ok(children) = children_query.get(ui_id) {
//                 for &child in children.iter() {
//                     update_text_markers_recursive(child, storage, &mut texts, &children_query);
//                 }
//             }
//         }
//     }
// }
//
// fn update_text_markers_recursive(
//     entity: Entity,
//     storage: &Storage,
//     texts: &mut Query<(&mut Text, Option<&StorageOwnerUIMarker>), Or<(With<StorageOwnerUIMarker>,)>>,
//     children_query: &Query<&Children>,
// ) {
//     if let Ok((mut text, storage_owner_marker)) = texts.get_mut(entity) {
//         if storage_owner_marker.is_some() {
//             text.sections[0].value = storage_owner_text(storage);
//         }
//     }
//
//     if let Ok(children) = children_query.get(entity) {
//         for &child in children.iter() {
//             update_text_markers_recursive(child, storage, texts, children_query);
//         }
//     }
// }
//
// fn storage_owner_text(storage: &Storage) -> String {
//     format!("owner: {:?}", storage.owner)
// }
