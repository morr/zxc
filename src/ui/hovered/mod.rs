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
            );
    }
}

fn render_hovered_ui(mut commands: Commands) {
    // commands
    //     .spawn((
    //         NodeBundle {
    //             style: Style {
    //                 position_type: PositionType::Absolute,
    //                 display: Display::Flex,
    //                 flex_direction: FlexDirection::Row,
    //                 column_gap: Val::Px(25.),
    //                 bottom: UI_SCREEN_EDGE_PX_OFFSET,
    //                 left: UI_SCREEN_EDGE_PLUS_ITEM_STOCKS_PX_OFFSET,
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         HoveredContainerUIMarker::default(),
    //     ))
    //     .with_children(|parent| {
    //         parent
    //             .spawn(render_entity_node_bunlde::<TileUIMarker>())
    //
    //     });
}

fn selectble_id(container_query: &Query<Entity, With<HoveredContainerUIMarker>>) -> Entity {
    container_query.get_single().unwrap()
}

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
