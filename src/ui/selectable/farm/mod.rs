use self::structure::{spawn_farm, Farm, FarmState};

use super::*;

#[derive(Component, Default)]
pub struct FarmUIMarker {}
#[derive(Component, Default)]
pub struct FarmStateTextUIMarker {}
#[derive(Component, Default)]
pub struct FarmTendingsTextUIMarker {}
#[derive(Component, Default)]
pub struct FarmYieldTextUIMarker {}

#[derive(Component, Default)]
pub struct WorkableWorkAmountDoneTextUIMarker {}
#[derive(Component, Default)]
pub struct WorkableWorkAmountTotalTextUIMarker {}

pub struct UiFarmPlugin;

impl Plugin for UiFarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(AppState::Loading),
            render_farm_ui
                .after(render_selectable_container)
                .after(spawn_farm),
        )
        .add_systems(
            FixedUpdate,
            (update_farm_ui, update_workable_ui)
                .chain()
                .after(render_farm_ui)
                .run_if(in_state(AppState::Playing)),
        );
    }
}

fn render_farm_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    farm_query: Query<(&Farm, &Workable)>,
    container_query: Query<Entity, With<SelectableContainerUIMarker>>,
) {
    let (farm, workable) = farm_query.iter().next().unwrap();

    commands
        .entity(selectble_id(&container_query))
        .with_children(|parent| {
            parent
                .spawn(render_entity_node_bunlde::<FarmUIMarker>())
                .with_children(|parent| {
                    parent
                        .spawn(selectable_subnode_bunlde())
                        .with_children(|parent| {
                            parent.spawn(headline_text_bundle("Farm", &font_assets));
                            parent.spawn(property_text_bundle::<FarmYieldTextUIMarker>(
                                farm_yield_text(farm),
                                &font_assets,
                            ));
                            parent.spawn(property_text_bundle::<FarmTendingsTextUIMarker>(
                                farm_tendings_text(farm),
                                &font_assets,
                            ));
                            parent.spawn(property_text_bundle::<FarmStateTextUIMarker>(
                                farm_state_text(farm),
                                &font_assets,
                            ));
                        });

                    parent
                        .spawn(selectable_subnode_bunlde())
                        .with_children(|parent| {
                            parent.spawn(headline_text_bundle("Workable", &font_assets));
                            parent.spawn(
                                property_text_bundle::<WorkableWorkAmountDoneTextUIMarker>(
                                    workable_work_amount_done_text(workable),
                                    &font_assets,
                                ),
                            );
                            parent.spawn(
                                property_text_bundle::<WorkableWorkAmountTotalTextUIMarker>(
                                    workable_work_amount_total_text(workable),
                                    &font_assets,
                                ),
                            );
                        });
                });
        });
}

fn update_farm_ui(
    mut texts: Query<
        (
            &mut Text,
            Option<&FarmStateTextUIMarker>,
            Option<&FarmTendingsTextUIMarker>,
            Option<&FarmYieldTextUIMarker>,
        ),
        Or<(
            With<FarmStateTextUIMarker>,
            With<FarmTendingsTextUIMarker>,
            With<FarmYieldTextUIMarker>,
        )>,
    >,
    farm_query: Query<&Farm>,
) {
    let farm = farm_query.iter().next().unwrap();

    for (mut text, state_marker, tendings_marker, yield_marker) in texts.iter_mut() {
        if state_marker.is_some() {
            text.sections[0].value = farm_state_text(farm);
        } else if tendings_marker.is_some() {
            text.sections[0].value = farm_tendings_text(farm);
        } else if yield_marker.is_some() {
            text.sections[0].value = farm_yield_text(farm);
        }
    }
}

fn farm_state_text(farm: &Farm) -> String {
    match &farm.state {
        FarmState::Planted(planted_state) => format!("state: {:?}", structure::PlantedStateDebug(planted_state)),
        FarmState::Harvested(harvested_state) => format!("state: {:?}", structure::HarvestedStateDebug(harvested_state)),
        _ => format!("state: {:?}", farm.state),
    }
}

fn farm_tendings_text(farm: &Farm) -> String {
    format!("tendings: {}", farm.tendings_done)
}

fn farm_yield_text(farm: &Farm) -> String {
    format!("yield: {}", farm.yield_amount())
}

fn update_workable_ui(
    mut texts: Query<
        (
            &mut Text,
            Option<&WorkableWorkAmountDoneTextUIMarker>,
            Option<&WorkableWorkAmountTotalTextUIMarker>,
        ),
        Or<(
            With<WorkableWorkAmountDoneTextUIMarker>,
            With<WorkableWorkAmountTotalTextUIMarker>,
        )>,
    >,
    workable_query: Query<&Workable>,
) {
    let workable = workable_query.iter().next().unwrap();

    for (mut text, work_amount_done_marker, work_amount_total_marker) in texts.iter_mut() {
        if work_amount_done_marker.is_some() {
            text.sections[0].value = workable_work_amount_done_text(workable);
        } else if work_amount_total_marker.is_some() {
            text.sections[0].value = workable_work_amount_total_text(workable);
        }
    }
}

pub fn workable_work_amount_done_text(workable: &Workable) -> String {
    format!("work_amount_done: {:.2}", workable.work_amount_done)
}

pub fn workable_work_amount_total_text(workable: &Workable) -> String {
    format!("work_amount_total: {:.2}", workable.work_amount_total)
}
