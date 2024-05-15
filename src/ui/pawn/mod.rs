use bevy::ecs::system::EntityCommands;

use super::*;

#[derive(Component)]
pub struct PawnUIMarker {
    pawn_id: Entity,
}

impl TargetEntityUiMarker for PawnUIMarker {
    fn new(pawn_id: Entity) -> Self {
        Self { pawn_id }
    }
}

#[derive(Component, Default)]
pub struct PawnComponentUIMarker {}

#[derive(Component, Default)]
struct PawnAgeTextUIMarker {}
#[derive(Component, Default)]
struct PawnLifetimeTextUIMarker {}
#[derive(Component, Default)]
struct PawnBirthdayTextUIMarker {}
#[derive(Component, Default)]
struct PawnStateTextUIMarker {}

#[derive(Component, Default)]
pub struct MovableComponentUIMarker {}

#[derive(Component, Default)]
struct MovableSpeedTextUIMarker {}
#[derive(Component, Default)]
struct MovablePathTextUIMarker {}
#[derive(Component, Default)]
struct MovableStateTextUIMarker {}

pub struct UiPawnPlugin;

impl Plugin for UiPawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            update_pawn_ui.run_if(in_state(AppState::Playing)),
        );
    }
}

fn update_pawn_ui(
    ui_query: Query<(Entity, &PawnUIMarker)>,
    mut texts: Query<
        (
            &mut Text,
            Option<&PawnAgeTextUIMarker>,
            Option<&PawnLifetimeTextUIMarker>,
            Option<&PawnBirthdayTextUIMarker>,
            Option<&PawnStateTextUIMarker>,
            Option<&MovableSpeedTextUIMarker>,
            Option<&MovablePathTextUIMarker>,
            Option<&MovableStateTextUIMarker>,
        ),
        Or<(
            With<PawnAgeTextUIMarker>,
            With<PawnLifetimeTextUIMarker>,
            With<PawnBirthdayTextUIMarker>,
            With<PawnStateTextUIMarker>,
            With<MovableSpeedTextUIMarker>,
            With<MovablePathTextUIMarker>,
            With<MovableStateTextUIMarker>,
        )>,
    >,
    components_query: Query<(&Pawn, &Movable)>,
    children_query: Query<&Children>,
) {
    for (ui_id, ui_marker) in ui_query.iter() {
        if let Ok((pawn, movable)) = components_query.get(ui_marker.pawn_id) {
            if let Ok(children) = children_query.get(ui_id) {
                for &child in children.iter() {
                    update_text_markers_recursive(
                        child,
                        pawn,
                        movable,
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
    pawn: &Pawn,
    movable: &Movable,
    texts: &mut Query<
        (
            &mut Text,
            Option<&PawnAgeTextUIMarker>,
            Option<&PawnLifetimeTextUIMarker>,
            Option<&PawnBirthdayTextUIMarker>,
            Option<&PawnStateTextUIMarker>,
            Option<&MovableSpeedTextUIMarker>,
            Option<&MovablePathTextUIMarker>,
            Option<&MovableStateTextUIMarker>,
        ),
        Or<(
            With<PawnAgeTextUIMarker>,
            With<PawnLifetimeTextUIMarker>,
            With<PawnBirthdayTextUIMarker>,
            With<PawnStateTextUIMarker>,
            With<MovableSpeedTextUIMarker>,
            With<MovablePathTextUIMarker>,
            With<MovableStateTextUIMarker>,
        )>,
    >,
    children_query: &Query<&Children>,
) {
    if let Ok((
        mut text,
        pawn_age_marker,
        pwan_lifetime_marker,
        pawn_birthday_marker,
        pwan_state_marker,
        movable_speed_marker,
        movable_path_marker,
        movable_state_marker,
    )) = texts.get_mut(entity)
    {
        if pawn_age_marker.is_some() {
            text.sections[0].value = pawn_age_text(pawn);
        }
        if pwan_lifetime_marker.is_some() {
            text.sections[0].value = pawn_lifetime_text(pawn);
        }
        if pawn_birthday_marker.is_some() {
            text.sections[0].value = pawn_birthday_text(pawn);
        }
        if pwan_state_marker.is_some() {
            text.sections[0].value = pawn_state_text(pawn);
        }
        if movable_speed_marker.is_some() {
            text.sections[0].value = movable_speed_text(movable);
        }
        if movable_path_marker.is_some() {
            text.sections[0].value = movable_path_text(movable);
        }
        if movable_state_marker.is_some() {
            text.sections[0].value = movable_state_text(movable);
        }
    }

    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            update_text_markers_recursive(child, pawn, movable, texts, children_query);
        }
    }
}

pub fn render_pawn_ui(
    pawn_id: Entity,
    container_ui_commands: &mut EntityCommands,
    pawn: &Pawn,
    movable: &Movable,
    font_assets: &Res<FontAssets>,
    opacity: UiOpacity,
) {
    container_ui_commands.with_children(|parent| {
        parent
            .spawn(render_entity_node_bunlde::<PawnUIMarker>(pawn_id, opacity))
            .with_children(|parent| {
                parent
                    .spawn(render_entity_component_node_bunlde::<PawnComponentUIMarker>())
                    .with_children(|parent| {
                        parent.spawn(headline_text_bundle(format!("Pawn {:?}", pawn_id), font_assets));
                        parent.spawn(property_text_bundle::<PawnAgeTextUIMarker>(
                            pawn_age_text(pawn),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<PawnLifetimeTextUIMarker>(
                            pawn_lifetime_text(pawn),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<PawnBirthdayTextUIMarker>(
                            pawn_birthday_text(pawn),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<PawnStateTextUIMarker>(
                            pawn_state_text(pawn),
                            font_assets,
                        ));
                    });

                parent
                    .spawn(render_entity_component_node_bunlde::<
                        MovableComponentUIMarker,
                    >())
                    .with_children(|parent| {
                        parent.spawn(headline_text_bundle("Movable".into(), font_assets));
                        parent.spawn(property_text_bundle::<MovableSpeedTextUIMarker>(
                            movable_speed_text(movable),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<MovablePathTextUIMarker>(
                            movable_path_text(movable),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<MovableStateTextUIMarker>(
                            movable_state_text(movable),
                            font_assets,
                        ));
                    });
            });
    });
}

fn pawn_age_text(pawn: &Pawn) -> String {
    format!("age: {}", pawn.age)
}
fn pawn_lifetime_text(pawn: &Pawn) -> String {
    if pawn.lifetime > CONFIG.time.day_duration {
        format!(
            "lifetime: {}y {}d",
            (pawn.lifetime / CONFIG.time.year_duration).floor(),
            ((pawn.lifetime % CONFIG.time.year_duration) / CONFIG.time.day_duration).floor()
        )
    } else {
        // hours displayed in this case only because only in the last day we track lifetime by
        // miniMal chunk of time
        format!(
            "lifetime: 0y 0d {}h {}m",
            (pawn.lifetime / CONFIG.time.hour_duration).floor(),
            ((pawn.lifetime % CONFIG.time.hour_duration) / CONFIG.time.minute_duration).floor()
        )
    }
}
fn pawn_birthday_text(pawn: &Pawn) -> String {
    format!(
        "birthday: {}",
        ElapsedTime::year_day_to_season_day_label(pawn.birth_year_day)
    )
}
fn pawn_state_text(pawn: &Pawn) -> String {
    format!("state: {:?}", pawn.state)
}

fn movable_speed_text(movable: &Movable) -> String {
    format!("speed: {}", movable.speed)
}
fn movable_path_text(movable: &Movable) -> String {
    format!("path: {:?}", movable.path)
}
fn movable_state_text(movable: &Movable) -> String {
    format!("state: {:?}", movable.state)
}
