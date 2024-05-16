use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct HoveredGridTile(pub Option<IVec2>);

#[derive(Component, Default)]
pub struct HoverMarker;

#[derive(Event, Debug)]
pub struct HoverEvent(pub IVec2);

#[derive(Event, Debug)]
pub struct ClickEventStage0(pub IVec2);

#[derive(Event, Debug)]
pub struct ClickEventStage1(pub IVec2);

