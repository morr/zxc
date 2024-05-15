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
        ))
        .with_children(|parent| {
            parent
                .spawn(render_entity_node_bunlde::<TileUIMarker>(Display::None))
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
        });
}

// fn selectble_id(container_query: &Query<Entity, With<HoveredContainerUIMarker>>) -> Entity {
//     container_query.get_single().unwrap()
// }

fn update_ui_on_hover_event(
    mut hover_event_reader: EventReader<HoverEvent>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    for event in hover_event_reader.read() {
        let navmesh = arc_navmesh.read();
        for id in navmesh.get_entities::<Tile>(event.0.x, event.0.y) {
            // println!("{:?} {:?}", event, id);
        }
    }
}

fn update_tile_ui(//
) {
}
