use bevy::ecs::system::EntityCommands;

use super::*;

#[derive(Component, Default)]
pub struct PawnUIMarker {}

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
            (update_pawn_ui, update_movable_ui)
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

fn update_pawn_ui(
    q_ui: Query<Entity, With<PawnUIMarker>>
//     mut texts: Query<
//         (
//             &mut Text,
//             Option<&PawnAgeTextUIMarker>,
//             Option<&PawnLifetimeTextUIMarker>,
//             Option<&PawnBirthdayTextUIMarker>,
//             Option<&PawnStateTextUIMarker>,
//         ),
//         Or<(
//             With<PawnAgeTextUIMarker>,
//             With<PawnLifetimeTextUIMarker>,
//             With<PawnBirthdayTextUIMarker>,
//             With<PawnStateTextUIMarker>,
//         )>,
//     >,
    // pawn_query: Query<&Pawn>,
) {
    for ui_id in q_ui.iter() {
        println!("{:?}", ui_id);
    }
//     let pawn = pawn_query.iter().next().unwrap();
//
//     for (mut text, age_marker, lifetimer_marker, birthday_marker, state_marker) in texts.iter_mut()
//     {
//         if age_marker.is_some() {
//             text.sections[0].value = pawn_age_text(pawn);
//         } else if lifetimer_marker.is_some() {
//             text.sections[0].value = pawn_lifetime_text(pawn);
//         } else if birthday_marker.is_some() {
//             text.sections[0].value = pawn_birthday_text(pawn);
//         } else if state_marker.is_some() {
//             text.sections[0].value = pawn_state_text(pawn);
//         }
//     }
}


pub fn render_pawn_ui(
    id: &Entity,
    container_ui_commands: &mut EntityCommands,
    pawn: &Pawn,
    movable: &Movable,
    font_assets: &Res<FontAssets>,
) {
    container_ui_commands.with_children(|parent| {
        parent
            .spawn(render_entity_node_bunlde::<PawnUIMarker>())
            .with_children(|parent| {
                parent
                    .spawn(render_entity_component_node_bunlde::<PawnComponentUIMarker>())
                    .with_children(|parent| {
                        parent.spawn(headline_text_bundle("Pawn", font_assets));
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
                        parent.spawn(headline_text_bundle("Movable", font_assets));
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

fn update_movable_ui(
//     mut texts: Query<
//         (
//             &mut Text,
//             Option<&MovableSpeedTextUIMarker>,
//             Option<&MovablePathTextUIMarker>,
//             Option<&MovableStateTextUIMarker>,
//         ),
//         Or<(
//             With<MovableSpeedTextUIMarker>,
//             With<MovablePathTextUIMarker>,
//             With<MovableStateTextUIMarker>,
//         )>,
//     >,
//     movable_query: Query<&Movable>,
) {
//     let movable = movable_query.iter().next().unwrap();
//
//     for (mut text, speed_marker, pathr_marker, state_marker) in texts.iter_mut() {
//         if speed_marker.is_some() {
//             text.sections[0].value = movable_speed_text(movable);
//         } else if pathr_marker.is_some() {
//             text.sections[0].value = movable_path_text(movable);
//         } else if state_marker.is_some() {
//             text.sections[0].value = movable_state_text(movable);
//         }
//     }
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
