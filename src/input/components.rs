use bevy::prelude::*;

// #[derive(Resource, Default)]
// pub struct PrevHoveredTilePos;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct PrevHoveredTilePos(pub Option<UVec2>);
