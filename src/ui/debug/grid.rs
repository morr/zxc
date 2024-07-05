use super::*;

pub struct DebugGridPlugin;
impl Plugin for DebugGridPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<DebugGridState>().add_systems(
            Update,
            render_grid
                .run_if(in_state(AppState::Playing))
                .run_if(in_state(DebugGridState::Visible)),
        );
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum DebugGridState {
    #[default]
    Hidden,
    Visible,
}

pub fn render_grid(mut gizmos: Gizmos) {
    for i in -config().grid.half_size..config().grid.half_size {
        gizmos.line_2d(
            Vec2::new(
                grid_tile_edge_to_world(-config().grid.half_size),
                grid_tile_edge_to_world(i),
            ),
            Vec2::new(
                grid_tile_edge_to_world(config().grid.half_size),
                grid_tile_edge_to_world(i),
            ),
            Color::srgba(0.2, 0.2, 0.2, 0.5),
        );
    }

    for i in -config().grid.half_size..config().grid.half_size {
        gizmos.line_2d(
            Vec2::new(
                grid_tile_edge_to_world(i),
                grid_tile_edge_to_world(-config().grid.half_size),
            ),
            Vec2::new(
                grid_tile_edge_to_world(i),
                grid_tile_edge_to_world(config().grid.half_size),
            ),
            Color::srgba(0.2, 0.2, 0.2, 0.5),
        );
    }

    gizmos.line_2d(
        Vec2::new(0.0, 0.0),
        Vec2::new(config().grid.size as f32 * config().tile.size, 0.0),
        Color::rgb(1.0, 0.0, 0.0),
    );

    gizmos.line_2d(
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, config().grid.size as f32 * config().tile.size),
        Color::rgb(0.0, 1.0, 0.0),
    );
}
