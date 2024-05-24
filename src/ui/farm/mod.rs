use bevy::ecs::system::EntityCommands;

use super::*;

#[derive(Component)]
pub struct FarmUIMarker {
    farm_id: Entity,
}

impl TargetEntityUiMarker for FarmUIMarker {
    fn new(farm_id: Entity) -> Self {
        Self { farm_id }
    }
}

#[derive(Component, Default)]
pub struct FarmComponentUIMarker {}

#[derive(Component, Default)]
pub struct FarmStateTextUIMarker {}
#[derive(Component, Default)]
pub struct FarmTendingsTextUIMarker {}
#[derive(Component, Default)]
pub struct FarmYieldTextUIMarker {}

#[derive(Component, Default)]
pub struct WorkableComponentUIMarker {}

#[derive(Component, Default)]
pub struct WorkableStateTextUIMarker {}
#[derive(Component, Default)]
pub struct WorkableWorkAmountDoneTextUIMarker {}
#[derive(Component, Default)]
pub struct WorkableWorkAmountTotalTextUIMarker {}

pub struct UiFarmPlugin;

impl Plugin for UiFarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_farm_ui.run_if(in_state(AppState::Playing)),
        );
    }
}

pub fn render_farm_ui(
    farm_id: Entity,
    container_ui_commands: &mut EntityCommands,
    farm: &Farm,
    workable: &Workable,
    font_assets: &Res<FontAssets>,
    opacity: UiOpacity,
) {
    container_ui_commands.with_children(|parent| {
        parent
            .spawn(render_entity_node_bunlde::<FarmUIMarker>(farm_id, opacity))
            .with_children(|parent| {
                parent
                    .spawn(render_entity_component_node_bunlde::<FarmComponentUIMarker>())
                    .with_children(|parent| {
                        parent.spawn(headline_text_bundle(format!("Farm {:?}", farm_id), font_assets));
                        parent.spawn(property_text_bundle::<FarmYieldTextUIMarker>(
                            farm_yield_text(farm),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<FarmTendingsTextUIMarker>(
                            farm_tendings_text(farm),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<FarmStateTextUIMarker>(
                            farm_state_text(farm),
                            font_assets,
                        ));
                    });

                parent
                    .spawn(render_entity_component_node_bunlde::<
                        WorkableComponentUIMarker,
                    >())
                    .with_children(|parent| {
                        parent.spawn(headline_text_bundle("Workable".into(), font_assets));
                        parent.spawn(property_text_bundle::<WorkableStateTextUIMarker>(
                            workable_state_text(workable),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<WorkableWorkAmountDoneTextUIMarker>(
                            workable_amount_done_text(workable),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<WorkableWorkAmountTotalTextUIMarker>(
                            workable_amount_total_text(workable),
                            font_assets,
                        ));
                    });
            });
    });
}

fn update_farm_ui(
    ui_query: Query<(Entity, &FarmUIMarker)>,
    mut texts: Query<
        (
            &mut Text,
            Option<&FarmStateTextUIMarker>,
            Option<&FarmTendingsTextUIMarker>,
            Option<&FarmYieldTextUIMarker>,
            Option<&WorkableStateTextUIMarker>,
            Option<&WorkableWorkAmountDoneTextUIMarker>,
            Option<&WorkableWorkAmountTotalTextUIMarker>,
        ),
        Or<(
            With<FarmStateTextUIMarker>,
            With<FarmTendingsTextUIMarker>,
            With<FarmYieldTextUIMarker>,
            With<WorkableStateTextUIMarker>,
            With<WorkableWorkAmountDoneTextUIMarker>,
            With<WorkableWorkAmountTotalTextUIMarker>,
        )>,
    >,
    components_query: Query<(&Farm, &Workable)>,
    children_query: Query<&Children>,
) {
    for (ui_id, ui_marker) in ui_query.iter() {
        if let Ok((farm, workable)) = components_query.get(ui_marker.farm_id) {
            if let Ok(children) = children_query.get(ui_id) {
                for &child in children.iter() {
                    update_text_markers_recursive(
                        child,
                        farm,
                        workable,
                        &mut texts,
                        &children_query,
                    );
                }
            }
        }
    }
}

fn update_text_markers_recursive(
    entity: Entity,
    farm: &Farm,
    workable: &Workable,
    texts: &mut Query<
        (
            &mut Text,
            Option<&FarmStateTextUIMarker>,
            Option<&FarmTendingsTextUIMarker>,
            Option<&FarmYieldTextUIMarker>,
            Option<&WorkableStateTextUIMarker>,
            Option<&WorkableWorkAmountDoneTextUIMarker>,
            Option<&WorkableWorkAmountTotalTextUIMarker>,
        ),
        Or<(
            With<FarmStateTextUIMarker>,
            With<FarmTendingsTextUIMarker>,
            With<FarmYieldTextUIMarker>,
            With<WorkableStateTextUIMarker>,
            With<WorkableWorkAmountDoneTextUIMarker>,
            With<WorkableWorkAmountTotalTextUIMarker>,
        )>,
    >,
    children_query: &Query<&Children>,
) {
    if let Ok((
        mut text,
        farm_state_marker,
        farm_tendings_marker,
        farm_yield_marker,
        workable_state_marker,
        workable_amount_done_marker,
        workable_amount_total_marker,
    )) = texts.get_mut(entity)
    {
        if farm_state_marker.is_some() {
            text.sections[0].value = farm_state_text(farm);
        }
        if farm_tendings_marker.is_some() {
            text.sections[0].value = farm_tendings_text(farm);
        }
        if farm_yield_marker.is_some() {
            text.sections[0].value = farm_yield_text(farm);
        }
        if workable_state_marker.is_some() {
            text.sections[0].value = workable_state_text(workable);
        }
        if workable_amount_done_marker.is_some() {
            text.sections[0].value = workable_amount_done_text(workable);
        }
        if workable_amount_total_marker.is_some() {
            text.sections[0].value = workable_amount_total_text(workable);
        }
    }

    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            update_text_markers_recursive(child, farm, workable, texts, children_query);
        }
    }
}

fn farm_state_text(farm: &Farm) -> String {
    match &farm.state {
        FarmState::Planted(planted_state) => {
            format!("state: {:?}", structure::PlantedStateDebug(planted_state))
        }
        FarmState::Harvested(harvested_state) => format!(
            "state: {:?}",
            structure::HarvestedStateDebug(harvested_state)
        ),
        _ => format!("state: {:?}", farm.state),
    }
}

fn farm_tendings_text(farm: &Farm) -> String {
    format!("tendings: {}", farm.tendings_done)
}

fn farm_yield_text(farm: &Farm) -> String {
    format!("yield: {}", farm.yield_amount())
}

pub fn workable_state_text(workable: &Workable) -> String {
    format!("state: {:?}", workable.state)
}
pub fn workable_amount_done_text(workable: &Workable) -> String {
    format!("amount_done: {:.2}", workable.amount_done)
}
pub fn workable_amount_total_text(workable: &Workable) -> String {
    format!("amount_total: {:.2}", workable.amount_total)
}
