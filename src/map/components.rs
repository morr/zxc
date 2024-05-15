use super::*;

#[derive(Component, Debug)]
pub struct Tile(pub IVec2);

// #[derive(Component)]
// pub struct Actor;
//
// #[derive(Default, Component, Debug)]
// pub struct Pathing {
//     pub target_position: Option<Vec2>,
//     pub metadata: Option<RouteMetadata>,
//     pub portal_route: Option<Vec<(SectorID, FieldCell)>>,
//     pub has_los: bool,
// }
