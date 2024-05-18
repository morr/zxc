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

#[derive(Component, Default)]
pub struct RestableComponentUIMarker {}

#[derive(Component, Default)]
struct RestableStaminaTextUIMarker {}

#[derive(Component, Default)]
pub struct CommandableComponentUIMarker {}

#[derive(Component, Default)]
struct CommandableStateTextUIMarker {}
#[derive(Component, Default)]
struct CommandableQueueTextUIMarker {}

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
            Option<&RestableStaminaTextUIMarker>,
            Option<&CommandableStateTextUIMarker>,
            Option<&CommandableQueueTextUIMarker>,
        ),
        Or<(
            With<PawnAgeTextUIMarker>,
            With<PawnLifetimeTextUIMarker>,
            With<PawnBirthdayTextUIMarker>,
            With<PawnStateTextUIMarker>,
            With<MovableSpeedTextUIMarker>,
            With<MovablePathTextUIMarker>,
            With<MovableStateTextUIMarker>,
            With<RestableStaminaTextUIMarker>,
            With<CommandableStateTextUIMarker>,
            With<CommandableQueueTextUIMarker>,
        )>,
    >,
    components_query: Query<(&Pawn, &Movable, &Restable, &Commandable)>,
    children_query: Query<&Children>,
) {
    for (ui_id, ui_marker) in ui_query.iter() {
        if let Ok((pawn, movable, restable, commandable)) = components_query.get(ui_marker.pawn_id)
        {
            if let Ok(children) = children_query.get(ui_id) {
                for &child in children.iter() {
                    update_text_markers_recursive(
                        child,
                        pawn,
                        movable,
                        restable,
                        commandable,
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
    restable: &Restable,
    commandable: &Commandable,
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
            Option<&RestableStaminaTextUIMarker>,
            Option<&CommandableStateTextUIMarker>,
            Option<&CommandableQueueTextUIMarker>,
        ),
        Or<(
            With<PawnAgeTextUIMarker>,
            With<PawnLifetimeTextUIMarker>,
            With<PawnBirthdayTextUIMarker>,
            With<PawnStateTextUIMarker>,
            With<MovableSpeedTextUIMarker>,
            With<MovablePathTextUIMarker>,
            With<MovableStateTextUIMarker>,
            With<RestableStaminaTextUIMarker>,
            With<CommandableStateTextUIMarker>,
            With<CommandableQueueTextUIMarker>,
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
        restable_stamina_marker,
        commandable_state_marker,
        commandable_queue_marker,
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
        if restable_stamina_marker.is_some() {
            text.sections[0].value = restable_stamina_text(restable);
        }
        if commandable_state_marker.is_some() {
            text.sections[0].value = commandable_state_text(commandable);
        }
        if commandable_queue_marker.is_some() {
            text.sections[0].value = commandable_queue_text(commandable);
        }
    }

    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            update_text_markers_recursive(child, pawn, movable, restable, commandable, texts, children_query);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn render_pawn_ui(
    pawn_id: Entity,
    container_ui_commands: &mut EntityCommands,
    pawn: &Pawn,
    movable: &Movable,
    restable: &Restable,
    commandable: &Commandable,
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
                        parent.spawn(headline_text_bundle(
                            format!("Pawn {:?}", pawn_id),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<PawnAgeTextUIMarker>(
                            pawn_age_text(pawn),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<PawnLifetimeTextUIMarker>(
                            pawn_lifetime_text(pawn),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<PawnStateTextUIMarker>(
                            pawn_state_text(pawn),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<PawnBirthdayTextUIMarker>(
                            pawn_birthday_text(pawn),
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

                parent
                    .spawn(render_entity_component_node_bunlde::<
                        RestableComponentUIMarker,
                    >())
                    .with_children(|parent| {
                        parent.spawn(headline_text_bundle("Restable".into(), font_assets));
                        parent.spawn(property_text_bundle::<RestableStaminaTextUIMarker>(
                            restable_stamina_text(restable),
                            font_assets,
                        ));
                    });

                parent
                    .spawn(render_entity_component_node_bunlde::<
                        CommandableComponentUIMarker,
                    >())
                    .with_children(|parent| {
                        parent.spawn(headline_text_bundle("Commandable".into(), font_assets));
                        parent.spawn(property_text_bundle::<CommandableStateTextUIMarker>(
                            commandable_state_text(commandable),
                            font_assets,
                        ));
                        parent.spawn(property_text_bundle::<CommandableQueueTextUIMarker>(
                            commandable_queue_text(commandable),
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
    match &pawn.state {
        PawnState::Working(working_state) => {
            format!("state: {:?}", WorkingStateDebug(working_state))
        }
        _ => format!("state: {:?}", pawn.state),
    }
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

fn restable_stamina_text(restable: &Restable) -> String {
    format!("stamina: {:.2}", restable.stamina)
}

fn commandable_state_text(commandable: &Commandable) -> String {
    format!("state: {:?}", commandable.state)
}
fn commandable_queue_text(commandable: &Commandable) -> String {
    format!("queue: {:?}", commandable.queue)
}
