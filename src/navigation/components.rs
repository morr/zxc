use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct PathfindingRequestEvent {
    pub from: Vec2,
    pub to: Vec2,
    // pub entity: Entity,
}
