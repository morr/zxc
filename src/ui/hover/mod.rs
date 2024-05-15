use bevy::ecs::entity;

use super::*;

#[derive(Component, Default)]
struct HoverContainerUIMarker {}

pub struct UiHoverMarkerPlugin;

impl Plugin for UiHoverMarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_hovered_ui)
            .add_systems(
                FixedUpdate,
                update_ui_on_hover_event.run_if(in_state(AppState::Playing)),
            );
        // .add_systems(
        //     FixedUpdate,
        //     (update_tile_ui).chain().run_if(in_state(AppState::Playing)),
        // );
    }
}

fn render_hovered_ui(mut commands: Commands) {
    commands.spawn((
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
        HoverContainerUIMarker::default(),
    ));
}

fn update_ui_on_hover_event(
    mut commands: Commands,
    mut hover_event_reader: EventReader<HoverEvent>,
    hover_container_ui_query: Query<Entity, With<HoverContainerUIMarker>>,
    font_assets: Res<FontAssets>,
    pawn_query: Query<(&Pawn, &Movable)>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    for event in hover_event_reader.read() {
        let navmesh = arc_navmesh.read();
        let hover_container_ui_id = hover_container_ui_query.get_single().unwrap();
        let mut hover_container_ui_commands = commands.entity(hover_container_ui_id);

        hover_container_ui_commands.despawn_descendants();

        for _tile_id in navmesh.get_entities::<Tile>(event.0.x, event.0.y) {
            render_tile_ui(&mut hover_container_ui_commands, event.0, &font_assets);
        }

        for pawn_id in navmesh.get_entities::<Pawn>(event.0.x, event.0.y) {
            if let Ok((pawn, movable)) = pawn_query.get(*pawn_id) {
                render_pawn_ui(&mut hover_container_ui_commands, pawn, movable, &font_assets);
            }
        }
    }
}
