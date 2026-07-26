use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct StateChangeEvent<T>(pub T);

#[derive(EntityEvent, Debug)]
pub struct EntityStateChangeEvent<T>(#[event_target] pub Entity, pub T);
