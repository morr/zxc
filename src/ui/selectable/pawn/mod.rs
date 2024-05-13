use super::*;

#[derive(Component, Default)]
struct PawnUIMarker {}

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
            render_pawn_ui
                .after(render_selectable_container)
                .after(spawn_pawns),
        )
        .add_systems(
            FixedUpdate,
            update_pawn_ui
                .after(render_pawn_ui)
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
    let pawn = pawn_query.iter().next().unwrap();

    commands
        .entity(selectble_id(&container_query))
        .with_children(|parent| {
            parent
                .spawn(selectable_node_bunlde::<PawnUIMarker>())
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
                            age_text(pawn),
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
                            lifetime_text(pawn),
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
                            birthday_text(pawn),
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

fn update_pawn_ui(
    mut texts: Query<
        (
            &mut Text,
            Option<&PawnAgeTextUIMarker>,
            Option<&PawnLifetimeTextUIMarker>,
            Option<&PawnBirthdayTextUIMarker>,
        ),
        Or<(
            With<PawnAgeTextUIMarker>,
            With<PawnLifetimeTextUIMarker>,
            With<PawnBirthdayTextUIMarker>,
        )>,
    >,
    pawn_query: Query<&Pawn>,
) {
    let pawn = pawn_query.iter().next().unwrap();

    for (mut text, age_marker, lifetimer_marker, birthday_marker) in texts.iter_mut() {
        if age_marker.is_some() {
            text.sections[0].value = age_text(pawn);
        } else if lifetimer_marker.is_some() {
            text.sections[0].value = lifetime_text(pawn);
        } else if birthday_marker.is_some() {
            text.sections[0].value = birthday_text(pawn);
        }
    }
}

fn age_text(pawn: &Pawn) -> String {
    format!("Age: {}", pawn.age)
}

fn lifetime_text(pawn: &Pawn) -> String {
    if pawn.state == PawnState::Dead {
        "<DEAD>".into()
    } else {
        format!(
            "Lifetime: {}y {}d",
            (pawn.lifetime / CONFIG.time.year_duration).floor(),
            ((pawn.lifetime % CONFIG.time.year_duration) / CONFIG.time.day_duration).floor()
        )
    }
}

fn birthday_text(pawn: &Pawn) -> String {
    format!(
        "Birthday: {}",
        ElapsedTime::year_day_to_season_day_label(pawn.birth_year_day)
    )
}
