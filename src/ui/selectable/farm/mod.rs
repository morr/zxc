use self::structure::{spawn_farm, Farm};

use super::*;

#[derive(Component, Default)]
pub struct FarmStateTextUIMarker {}

#[derive(Component, Default)]
pub struct FarmTendingsTextUIMarker {}

pub struct UiFarmPlugin;

impl Plugin for UiFarmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(AppState::Loading),
            render_farm_ui.after(render_selectable_container).after(spawn_farm),
        )
        .add_systems(
            FixedUpdate,
            (update_farm_state_text, update_farm_tendings_text)
                .chain()
                .after(render_farm_ui)
                .run_if(in_state(AppState::Playing)),
        );
    }
}

fn render_farm_ui(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    farm_query: Query<&Farm>,
    container_query: Query<Entity, With<SelectableContainerUIMarker>>,
) {
    let farm = farm_query.iter().next().unwrap();

    commands
        .entity(selectble_id(&container_query))
        .with_children(|parent| {
            parent
                .spawn(selectable_node_bunlde())
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "FARM",
                        TextStyle {
                            font: font_assets.fira.clone(),
                            font_size: 18.,
                            color: Color::WHITE,
                        },
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            format_farm_state_text(farm),
                            TextStyle {
                                font: font_assets.fira.clone(),
                                font_size: 16.,
                                color: Color::WHITE,
                            },
                        ),
                        FarmStateTextUIMarker::default(),
                    ));
                    parent.spawn((
                        TextBundle::from_section(
                            format_farm_tendings_text(farm),
                            TextStyle {
                                font: font_assets.fira.clone(),
                                font_size: 16.,
                                color: Color::WHITE,
                            },
                        ),
                        FarmTendingsTextUIMarker::default(),
                    ));
                });
        });
}

fn update_farm_state_text(
    mut text_query: Query<&mut Text, With<FarmStateTextUIMarker>>,
    farm_query: Query<&Farm>,
) {
    let mut text = text_query.single_mut();
    let farm = farm_query.iter().next().unwrap();

    text.sections[0].value = format_farm_state_text(farm);
}

fn format_farm_state_text(farm: &Farm) -> String {
    format!("{:?}", farm.state)
}

fn format_farm_tendings_text(farm: &Farm) -> String {
    format!("Tendings: {}", farm.tendings_done)
}

fn update_farm_tendings_text(
    mut text_query: Query<&mut Text, With<FarmTendingsTextUIMarker>>,
    farm_query: Query<&Farm>,
) {
    let mut text = text_query.single_mut();
    let farm = farm_query.iter().next().unwrap();

    text.sections[0].value = format_farm_tendings_text(farm);
}
