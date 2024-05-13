use super::*;

#[derive(Component, Default)]
struct PawnAgeTextUIMarker {}

#[derive(Component, Default)]
struct PawnLifetimeTextUIMarker {}

#[derive(Component, Default)]
struct PawnBirthdayTextUIMarker {}

pub struct UiPawnPlugin;

impl Plugin for UiPawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(AppState::Loading),
            render_pawn_ui.after(render_selectable_container),
        )
        .add_systems(
            FixedUpdate,
            (
                update_pawn_age_text,
                update_pawn_lifetime_text,
                update_pawn_birthday_text,
            )
                .chain()
                .run_if(in_state(AppState::Playing)),
        );
    }
}

fn render_pawn_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    pawn_query: Query<&Pawn>,
    container_query: Query<Entity, With<SelectableContainerUIMarker>>,
) {
    let pawn = pawn_query.iter().next();
    let container_id = container_query.get_single().unwrap();

    commands.entity(container_id).with_children(|parent| {
        parent
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect {
                        top: Val::Px(10.0),
                        right: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        left: Val::Px(10.0),
                    },
                    ..default()
                },
                background_color: bg_color(UiOpacity::Medium),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "PAWN",
                    TextStyle {
                        font: font_assets.fira.clone(),
                        font_size: 18.,
                        color: Color::WHITE,
                    },
                ));
                parent.spawn((
                    TextBundle::from_section(
                        format_pawn_age_text(pawn),
                        TextStyle {
                            font: font_assets.fira.clone(),
                            font_size: 16.,
                            color: Color::WHITE,
                        },
                    ),
                    PawnAgeTextUIMarker::default(),
                ));
                parent.spawn((
                    TextBundle::from_section(
                        format_pawn_lifetime_text(pawn),
                        TextStyle {
                            font: font_assets.fira.clone(),
                            font_size: 16.,
                            color: Color::WHITE,
                        },
                    ),
                    PawnLifetimeTextUIMarker::default(),
                ));
                parent.spawn((
                    TextBundle::from_section(
                        format_pawn_birthday_text(pawn),
                        TextStyle {
                            font: font_assets.fira.clone(),
                            font_size: 16.,
                            color: Color::WHITE,
                        },
                    ),
                    PawnBirthdayTextUIMarker::default(),
                ));
            });
    });
}

fn update_pawn_age_text(
    mut text_query: Query<&mut Text, With<PawnAgeTextUIMarker>>,
    pawn_query: Query<&Pawn>,
) {
    let mut text = text_query.single_mut();
    let pawn = pawn_query.iter().next();

    text.sections[0].value = format_pawn_age_text(pawn);
}

fn format_pawn_age_text(maybe_pawn: Option<&Pawn>) -> String {
    if let Some(pawn) = maybe_pawn {
        format!("Age: {}", pawn.age)
    } else {
        "<NONE>".into()
    }
}
fn update_pawn_lifetime_text(
    mut text_query: Query<&mut Text, With<PawnLifetimeTextUIMarker>>,
    pawn_query: Query<&Pawn>,
) {
    let mut text = text_query.single_mut();
    let pawn = pawn_query.iter().next();

    text.sections[0].value = format_pawn_lifetime_text(pawn);
}

fn format_pawn_lifetime_text(maybe_pawn: Option<&Pawn>) -> String {
    if let Some(pawn) = maybe_pawn {
        if pawn.state == PawnState::Dead {
            "<DEAD>".into()
        } else {
            format!(
                "Lifetime: {}y {}d",
                (pawn.lifetime / CONFIG.time.year_duration).floor(),
                ((pawn.lifetime % CONFIG.time.year_duration) / CONFIG.time.day_duration).floor()
            )
        }
    } else {
        "<NONE>".into()
    }
}

fn update_pawn_birthday_text(
    mut text_query: Query<&mut Text, With<PawnBirthdayTextUIMarker>>,
    pawn_query: Query<&Pawn>,
) {
    let mut text = text_query.single_mut();
    let pawn = pawn_query.iter().next();

    text.sections[0].value = format_pawn_birthday_text(pawn);
}

fn format_pawn_birthday_text(maybe_pawn: Option<&Pawn>) -> String {
    if let Some(pawn) = maybe_pawn {
        format!(
            "Birthday: {}",
            ElapsedTime::year_day_to_season_day_label(pawn.birth_year_day)
        )
    } else {
        "<NONE>".into()
    }
}
