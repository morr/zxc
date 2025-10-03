use bevy::ecs::system::EntityCommands;

use super::*;

#[derive(Component)]
pub struct CarryableUIMarker {
    #[allow(dead_code)]
    carryable_id: Entity,
}

impl TargetEntityUiMarker for CarryableUIMarker {
    fn new(carryable_id: Entity) -> Self {
        Self { carryable_id }
    }
}

#[derive(Component, Default)]
pub struct CarryableComponentUIMarker {}

#[derive(Component, Default)]
pub struct CarryableKindUIMarker {}

#[derive(Component, Default)]
pub struct CarryableAmountUIMarker {}

pub struct UiCarryablePlugin;

impl Plugin for UiCarryablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_carryable_ui.run_if(in_state(AppState::Playing)),
        );
    }
}

pub fn render_carryable_ui(
    carryable_id: Entity,
    container_ui_commands: &mut EntityCommands,
    carryable: &Carryable,
    font_assets: &Res<FontAssets>,
    opacity: UiOpacity,
) {
    container_ui_commands.with_children(|parent| {
        parent.spawn((
            render_entity_node_bunlde::<CarryableUIMarker>(carryable_id, opacity),
            children![(
                render_entity_component_node_bunlde::<CarryableComponentUIMarker>(),
                children![
                    headline_text_bundle(format!("Carryable {:?}", carryable_id), font_assets,),
                    property_text_bundle::<CarryableKindUIMarker>(
                        carryable_kind_text(carryable),
                        font_assets,
                    ),
                    property_text_bundle::<CarryableAmountUIMarker>(
                        carryable_amount_text(carryable),
                        font_assets,
                    ),
                ],
            ),],
        ));
    });
}

fn update_carryable_ui(
    ui_query: Query<(Entity, &CarryableUIMarker)>,
    texts_query: Query<
        (
            Entity,
            Option<&CarryableKindUIMarker>,
            Option<&CarryableAmountUIMarker>,
        ),
        Or<(With<CarryableKindUIMarker>, With<CarryableAmountUIMarker>)>,
    >,
    components_query: Query<&Carryable>,
    children_query: Query<&Children>,
    mut writer: TextUiWriter,
) {
    for (ui_id, ui_marker) in ui_query.iter() {
        if let Ok(carryable) = components_query.get(ui_marker.carryable_id) {
            if let Ok(children) = children_query.get(ui_id) {
                for child in children.iter() {
                    update_text_markers_recursive(
                        child,
                        carryable,
                        &texts_query,
                        &children_query,
                        &mut writer,
                    );
                }
            }
        }
    }
}

fn update_text_markers_recursive(
    entity: Entity,
    carryable: &Carryable,
    texts_query: &Query<
        (
            Entity,
            Option<&CarryableKindUIMarker>,
            Option<&CarryableAmountUIMarker>,
        ),
        Or<(With<CarryableKindUIMarker>, With<CarryableAmountUIMarker>)>,
    >,
    children_query: &Query<&Children>,
    writer: &mut TextUiWriter,
) {
    if let Ok((text_entity, carryable_kind_marker, carryable_amount_marker)) =
        texts_query.get(entity)
    {
        if carryable_kind_marker.is_some() {
            *writer.text(text_entity, 0) = carryable_kind_text(carryable);
        }
        if carryable_amount_marker.is_some() {
            *writer.text(text_entity, 0) = carryable_amount_text(carryable);
        }
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            update_text_markers_recursive(child, carryable, texts_query, children_query, writer);
        }
    }
}

fn carryable_kind_text(carryable: &Carryable) -> String {
    format!("kind: {:?}", carryable.kind)
}

fn carryable_amount_text(carryable: &Carryable) -> String {
    format!("amount: {:?}", carryable.amount)
}
