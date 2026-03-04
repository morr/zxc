use super::*;

#[derive(Component, Default)]
struct HoveredUIRootMarker {}

pub struct UiHoveredPlugin;

impl Plugin for UiHoveredPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::Loading), render_hovered_ui)
            .add_observer(on_hover_update_ui)
            .add_observer(on_occupation_change_update_ui);
    }
}

fn render_hovered_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::FlexEnd,
            column_gap: UI_WINDOWS_GAP,
            bottom: UI_SCREEN_EDGE_PX_OFFSET,
            left: UI_SCREEN_EDGE_PX_OFFSET,
            ..default()
        },
        HoveredUIRootMarker::default(),
    ));
}

#[allow(clippy::too_many_arguments)]
fn on_hover_update_ui(
    _event: On<HoverEvent>,
    commands: Commands,
    hovered_grid_tile: Res<HoveredGridTile>,
    hovered_root_ui_query: Query<Entity, With<HoveredUIRootMarker>>,
    pawn_query: Query<(&Pawn, &Movable, &Restable, &Feedable, &Commandable)>,
    farm_query: Query<(&Farm, &Workable)>,
    bed_query: Query<&Bed>,
    tile_query: Query<&Tile>,
    carryable_query: Query<&Carryable>,
    storage_query: Query<&Storage>,
    arc_navmesh: ResMut<ArcNavmesh>,
    font_assets: Res<FontAssets>,
) {
    let Some(grid_tile) = hovered_grid_tile.0 else {
        return;
    };
    redraw_hovered_ui(
        commands,
        grid_tile,
        hovered_root_ui_query,
        pawn_query,
        farm_query,
        bed_query,
        tile_query,
        carryable_query,
        storage_query,
        arc_navmesh,
        font_assets,
    );
}

#[allow(clippy::too_many_arguments)]
fn on_occupation_change_update_ui(
    event: On<OccupationChangeEvent>,
    commands: Commands,
    hovered_grid_tile: Res<HoveredGridTile>,
    hovered_root_ui_query: Query<Entity, With<HoveredUIRootMarker>>,
    pawn_query: Query<(&Pawn, &Movable, &Restable, &Feedable, &Commandable)>,
    farm_query: Query<(&Farm, &Workable)>,
    bed_query: Query<&Bed>,
    tile_query: Query<&Tile>,
    carryable_query: Query<&Carryable>,
    storage_query: Query<&Storage>,
    arc_navmesh: ResMut<ArcNavmesh>,
    font_assets: Res<FontAssets>,
) {
    let OccupationChangeEvent(changed_tiles) = &*event;
    let Some(grid_tile) = hovered_grid_tile.0 else {
        return;
    };
    if !changed_tiles.contains(&grid_tile) {
        return;
    }
    redraw_hovered_ui(
        commands,
        grid_tile,
        hovered_root_ui_query,
        pawn_query,
        farm_query,
        bed_query,
        tile_query,
        carryable_query,
        storage_query,
        arc_navmesh,
        font_assets,
    );
}

#[allow(clippy::too_many_arguments)]
fn redraw_hovered_ui(
    mut commands: Commands,
    grid_tile: IVec2,
    hovered_root_ui_query: Query<Entity, With<HoveredUIRootMarker>>,
    pawn_query: Query<(&Pawn, &Movable, &Restable, &Feedable, &Commandable)>,
    farm_query: Query<(&Farm, &Workable)>,
    bed_query: Query<&Bed>,
    tile_query: Query<&Tile>,
    carryable_query: Query<&Carryable>,
    storage_query: Query<&Storage>,
    arc_navmesh: ResMut<ArcNavmesh>,
    font_assets: Res<FontAssets>,
) {
    let hovered_root_ui_id = hovered_root_ui_query
        .single()
        .expect("HoveredUIRoot query failed");
    let mut hovered_root_ui_commands = commands.entity(hovered_root_ui_id);
    hovered_root_ui_commands.despawn_related::<Children>();

    let navmesh = arc_navmesh.read();

    // hover over Tile
    for tile_id in navmesh.get_occupants::<Tile>(grid_tile.x, grid_tile.y) {
        if let Ok(tile) = tile_query.get(*tile_id) {
            render_tile_ui(
                *tile_id,
                &mut hovered_root_ui_commands,
                tile,
                grid_tile,
                &font_assets,
                UiOpacity::Medium,
            );
        }
    }

    // hover over Bed
    for bed_id in navmesh.get_occupants::<Bed>(grid_tile.x, grid_tile.y) {
        if let Ok(bed) = bed_query.get(*bed_id) {
            render_bed_ui(
                *bed_id,
                &mut hovered_root_ui_commands,
                bed,
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
                &font_assets,
                UiOpacity::Medium,
            );
        }
    }

    // hover over Pawn
    for pawn_id in navmesh.get_occupants::<Pawn>(grid_tile.x, grid_tile.y) {
        if let Ok((pawn, movable, restable, feedable, commandable)) = pawn_query.get(*pawn_id) {
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

    // hover over Carryable
    for carryable_id in navmesh.get_occupants::<Carryable>(grid_tile.x, grid_tile.y) {
        if let Ok(carryable) = carryable_query.get(*carryable_id) {
            render_carryable_ui(
                *carryable_id,
                &mut hovered_root_ui_commands,
                carryable,
                &font_assets,
                UiOpacity::Medium,
            );
        }
    }
}
