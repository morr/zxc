use super::*;

#[derive(Component)]
struct PawnAgeTextUI {}

#[derive(Component)]
struct PawnLifetimeTextUI {}

#[derive(Component)]
struct PawnBirthdayTextUI {}

pub struct UiPawnPlugin;

impl Plugin for UiPawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_pawn_ui)
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
) {
    let pawn = pawn_query.iter().next();

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.),
                top: Val::Px(8.),
                left: Val::Px(100.),
                padding: UiRect {
                    top: Val::Px(10.),
                    right: Val::Px(10.),
                    bottom: Val::Px(10.),
                    left: Val::Px(10.),
                },
                ..default()
            },
            background_color: (*UI_COLOR.clone().set_a(0.85)).into(),
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
                PawnAgeTextUI {},
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
                PawnLifetimeTextUI {},
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
                PawnBirthdayTextUI {},
            ));
        });
}

fn update_pawn_age_text(
    mut text_query: Query<&mut Text, With<PawnAgeTextUI>>,
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
    mut text_query: Query<&mut Text, With<PawnLifetimeTextUI>>,
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
    mut text_query: Query<&mut Text, With<PawnBirthdayTextUI>>,
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
