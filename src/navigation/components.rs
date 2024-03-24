use bevy::{prelude::*, utils::HashSet};

#[derive(Debug, Default)]
pub struct NavTileOccupant {
    pub weight: f32,
    pub occupied_by: HashSet<Entity>,
    pub walkable: bool,
}

#[derive(Resource)]
pub struct Navmesh(pub Vec<Vec<NavTileOccupant>>);

// impl Default for Navmesh {
//     fn default() -> Self {
//         Self::default()
//     }
// }

#[derive(Debug, Event)]
pub struct PathfindingRequestEvent {
    pub from: Vec2,
    pub to: Vec2,
    // pub entity: Entity,
}
