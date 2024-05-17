use super::*;

#[derive(Component, Default)]
struct HoveredUIRootMarker {}

pub struct UiHoveredPlugin;

impl Plugin for UiHoveredPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_hovered_ui)
            .add_systems(
                Update,
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
    hovered_root_ui_query: Query<Entity, With<HoveredUIRootMarker>>,
    pawn_query: Query<(&Pawn, &Movable, &Restable)>,
    farm_query: Query<(&Farm, &Workable)>,
    arc_navmesh: ResMut<ArcNavmesh>,
    font_assets: Res<FontAssets>,
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
        let hovered_root_ui_id = hovered_root_ui_query.get_single().unwrap();
        let mut hovered_root_ui_commands = commands.entity(hovered_root_ui_id);
        hovered_root_ui_commands.despawn_descendants();

        let navmesh = arc_navmesh.read();

        // hover over Tile
        for tile_id in navmesh.get_occupation::<Tile>(grid_tile.x, grid_tile.y) {
            render_tile_ui(
                *tile_id,
                &mut hovered_root_ui_commands,
                *grid_tile,
                &font_assets,
                UiOpacity::Medium,
            );
        }

        // hover over Bed
        for tile_id in navmesh.get_occupation::<Bed>(grid_tile.x, grid_tile.y) {
            render_bed_ui(
                *tile_id,
                &mut hovered_root_ui_commands,
                *grid_tile,
                &font_assets,
                UiOpacity::Medium,
            );
        }

        // hover over Pawn
        for movable_id in navmesh.get_occupation::<Movable>(grid_tile.x, grid_tile.y) {
            if let Ok((pawn, movable, restable)) = pawn_query.get(*movable_id) {
                render_pawn_ui(
                    *movable_id,
                    &mut hovered_root_ui_commands,
                    pawn,
                    movable,
                    restable,
                    &font_assets,
                    UiOpacity::Medium,
                );
            }
        }

        // hover over Farm
        for farm_id in navmesh.get_occupation::<Farm>(grid_tile.x, grid_tile.y) {
            if let Ok((farm, workable)) = farm_query.get(*farm_id) {
                render_farm_ui(
                    *farm_id,
                    &mut hovered_root_ui_commands,
                    farm,
                    workable,
                    &font_assets,
                    UiOpacity::Medium,
                );
            }
        }
    }
}
