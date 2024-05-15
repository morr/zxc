use super::*;

#[derive(Component, Default)]
struct HoveredContainerUIMarker {}

pub struct UiHoveredMarkerPlugin;

impl Plugin for UiHoveredMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_hovered_ui)
            .add_systems(
                FixedUpdate,
                update_ui_on_hover_event.run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                FixedUpdate,
                (update_tile_ui).chain().run_if(in_state(AppState::Playing)),
            );
    }
}

fn render_hovered_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(25.),
                    bottom: UI_SCREEN_EDGE_PX_OFFSET,
                    left: UI_SCREEN_EDGE_PLUS_ITEM_STOCKS_PX_OFFSET,
                    ..default()
                },
                ..default()
            },
            HoveredContainerUIMarker::default(),
        ));
}

// fn selectble_id(container_query: &Query<Entity, With<HoveredContainerUIMarker>>) -> Entity {
//     container_query.get_single().unwrap()
// }

fn update_ui_on_hover_event(
    mut commands: Commands,
    mut hover_event_reader: EventReader<HoverEvent>,
    hovered_container_ui_query: Query<Entity, With<HoveredContainerUIMarker>>,
    tile_ui_query: Query<Entity, With<TileUIMarker>>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    for event in hover_event_reader.read() {
        // despawn all tile UIs
        for tile_ui_id in tile_ui_query.iter() {
            commands.entity(tile_ui_id).despawn_recursive();
        }

        let navmesh = arc_navmesh.read();
        for id in navmesh.get_entities::<Tile>(event.0.x, event.0.y) {
            let a = hovered_container_ui_query.get_single_mut().unwrap();
            // println!("{:?} {:?}", event, id);
        }
    }
}

fn update_tile_ui(//
) {
}

fn render_tile_ui(
) {
    parent
        .spawn(render_entity_node_bunlde::<TileUIMarker>())
        .with_children(|parent| {
            parent
                .spawn(render_entity_component_node_bunlde::<TileComponentUIMarker>())
                .with_children(|parent| {
                    parent.spawn(headline_text_bundle("Tile", &font_assets));
                    parent.spawn(property_text_bundle::<TileCoordsTextUIMarker>(
                        "".into(),
                        &font_assets,
                    ));
                });
        });
}
