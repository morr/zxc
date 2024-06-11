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
                column_gap: UI_WINDOWS_GAP,
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
    pawn_query: Query<(&Pawn, &Movable, &Restable, &Feedable, &Commandable)>,
    farm_query: Query<(&Farm, &Workable)>,
    bed_query: Query<&Bed>,
    storage_query: Query<&Storage>,
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
        for tile_id in navmesh.get_occupants::<Tile>(grid_tile.x, grid_tile.y) {
            render_tile_ui(
                *tile_id,
                &mut hovered_root_ui_commands,
                *grid_tile,
                &font_assets,
                UiOpacity::Medium,
            );
        }

        // hover over Bed
        for bed_id in navmesh.get_occupants::<Bed>(grid_tile.x, grid_tile.y) {
            if let Ok(bed) = bed_query.get(*bed_id) {
                render_bed_ui(
                    *bed_id,
                    &mut hovered_root_ui_commands,
                    bed,
                    *grid_tile,
                    &font_assets,
                    UiOpacity::Medium,
                );
            }
        }

        // hover over Storage
        for storage_id in navmesh.get_occupants::<Storage>(grid_tile.x, grid_tile.y) {
            if let Ok(storage) = storage_query.get(*storage_id) {
                render_storage_ui(
                    *storage_id,
                    &mut hovered_root_ui_commands,
                    storage,
                    *grid_tile,
                    &font_assets,
                    UiOpacity::Medium,
                );
            }
        }

        // hover over Pawn
        for pawn_id in navmesh.get_occupants::<Pawn>(grid_tile.x, grid_tile.y) {
            if let Ok((pawn, movable, restable, feedable, commandable)) =
                pawn_query.get(*pawn_id)
            {
                render_pawn_ui(
                    *pawn_id,
                    &mut hovered_root_ui_commands,
                    pawn,
                    movable,
                    restable,
                    feedable,
                    commandable,
                    &font_assets,
                    UiOpacity::Medium,
                );
            }
        }

        // hover over Farm
        for farm_id in navmesh.get_occupants::<Farm>(grid_tile.x, grid_tile.y) {
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
