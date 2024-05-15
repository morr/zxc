use self::structure::Farm;

use super::*;

#[derive(Component, Default)]
struct HoveredUIRootMarker {}

pub struct UiHoveredPlugin;

impl Plugin for UiHoveredPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_hovered_ui)
            .add_systems(
                FixedUpdate,
                update_ui_on_hover_event.run_if(in_state(AppState::Playing)),
            );
    }
}

fn render_hovered_ui(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexEnd,
                column_gap: Val::Px(25.),
                bottom: UI_SCREEN_EDGE_PX_OFFSET,
                left: UI_SCREEN_EDGE_PX_OFFSET,
                ..default()
            },
            ..default()
        },
        HoveredUIRootMarker::default(),
    ));
}

#[allow(clippy::too_many_arguments)]
fn update_ui_on_hover_event(
    mut commands: Commands,
    mut hover_event_reader: EventReader<HoverEvent>,
    hovered_grid_tile: Res<HoveredGridTile>,
    mut occupation_change_event_reader: EventReader<OccupationChangeEvent>,
    hover_container_ui_query: Query<Entity, With<HoveredUIRootMarker>>,
    font_assets: Res<FontAssets>,
    pawn_query: Query<(&Pawn, &Movable)>,
    farm_query: Query<(&Farm, &Workable)>,
    arc_navmesh: ResMut<ArcNavmesh>,
) {
    // have to redraw hover ui either on hover event
    let mut possibly_hovered_grid_tiles = hover_event_reader
        .read()
        .map(|v| v.0)
        .collect::<Vec<IVec2>>();

    // or if occupation has changed in navmesh
    for event in occupation_change_event_reader.read() {
        if let Some(grid_tile) = hovered_grid_tile.0 {
            if event.0.contains(&grid_tile) {
                possibly_hovered_grid_tiles.push(grid_tile);
            }
        }
    }

    if let Some(grid_tile) = possibly_hovered_grid_tiles.last() {
        let navmesh = arc_navmesh.read();
        let hover_container_ui_id = hover_container_ui_query.get_single().unwrap();
        let mut hover_container_ui_commands = commands.entity(hover_container_ui_id);

        hover_container_ui_commands.despawn_descendants();

        for target_id in navmesh.get_occupation::<Tile>(grid_tile.x, grid_tile.y) {
            render_tile_ui(
                *target_id,
                &mut hover_container_ui_commands,
                *grid_tile,
                &font_assets,
            );
        }

        for target_id in navmesh.get_occupation::<Movable>(grid_tile.x, grid_tile.y) {
            if let Ok((pawn, movable)) = pawn_query.get(*target_id) {
                render_pawn_ui(
                    *target_id,
                    &mut hover_container_ui_commands,
                    pawn,
                    movable,
                    &font_assets,
                );
            }
        }

        for target_id in navmesh.get_occupation::<Farm>(grid_tile.x, grid_tile.y) {
            if let Ok((farm, workable)) = farm_query.get(*target_id) {
                render_farm_ui(
                    *target_id,
                    &mut hover_container_ui_commands,
                    farm,
                    workable,
                    &font_assets,
                );
            }
        }
    }
}
